//! The page containing a map of pins indicating Chicago ShotSpotter alert locations.

use std::env;

use gloo_console::{error, log};
use gloo_net::http::Request;
use lazy_static::lazy_static;
use leaflet::{
    Icon, IconOptions, LatLng, LayerGroup, Map, MapOptions, Marker, MarkerOptions, Point, Popup,
    PopupOptions, TileLayer, TileLayerOptions,
};
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, Node};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    errors::StaccError,
    models::{
        chicago::{ChicagoMapData, CleanedShotData, CleanedViolenceData, ShotData, ViolenceData},
        response::Response,
    },
    pages::utils::Loading,
    traits::popup::Popup as PopupTrait,
    utils::{
        background,
        date::format_date,
        open_graph::{self, OpenGraphTag, PageType},
    },
    FAVICON_GIF,
};

use super::utils;

lazy_static! {
    /// City of Chicago data use disclaimer message. It is required to include this disclaimer on
    /// the site (https://www.chicago.gov/city/en/narr/foia/data_disclaimer.html).
    static ref CHICAGO_DATA_DISCLAIMER: &'static str = "This site provides applications using data that has been modified for use from its original source, www.cityofchicago.org, the official website of the City of Chicago.  The City of Chicago makes no claims as to the content, accuracy, timeliness, or completeness of any of the data provided at this site.  The data provided at this site is subject to change at any time.  It is understood that the data provided at this site is being used at one’s own risk.";

    /// Jawg.io maps attribution required to use the Jawg map API.
    static ref JAWG_MAPS_ATTRIBUTION: &'static str = r#"&copy; <a href="https://www.jawg.io/">JawgMaps</a> &copy; <a href="https://www.openstreetmap.org/copyright">OSM contributors</a>"#;
    /// The Leaflet attribution required to use the map API.
    static ref LEAFLET_ATTRIBUTION: &'static str = "<a href=\"https://www.jawg.io?utm_medium=map&utm_source=attribution\" target=\"_blank\">&copy; Jawg</a> - <a href=\"https://www.openstreetmap.org?utm_medium=map-attribution&utm_source=jawg\" target=\"_blank\">&copy; OpenStreetMap</a>&nbsp;contributors";
    /// The URL template for the Leaflet map.
    static ref LEAFLET_URL_TEMPLATE: String = {
        let leaflet_access_token = env::var("LEAFLET_ACCESS_TOKEN");
        if let Ok(access_token) = leaflet_access_token {
            let mut url_template = "https://tile.jawg.io/f6a80ab7-56ec-4b34-bc1c-3caec4328a77/{z}/{x}/{y}{r}.png?access-token=".to_string();
            url_template.push_str(&access_token);

            url_template
        } else {
            "????".to_string()
        }
    };

    /// The icon for mapping gunshot or firecracker alerts.
    static ref GUNSHOT_OR_FIRECRACKER_ICON: &'static str = "https://i.imgur.com/UalxwUV.png";
    /// The icon for mapping ShotSpotter alerts (single shot).
    static ref SINGLE_SHOTSPOTTER_MARKER_ICON: &'static str = "https://i.imgur.com/muz4yax.png";
    /// The icon for mapping ShotSpotter alerts (multiple shots).
    static ref MULTIPLE_SHOTSPOTTER_MARKER_ICON: &'static str = "https://i.imgur.com/3auFaRW.png";
    /// The icon for mapping violence alerts.
    static ref VIOLENCE_MARKER_ICON: &'static str = "https://i.imgur.com/5TV33u5.png";

    /// The City of Chicago Socrata API endpoint for data pertaining to victims of homicide and
    /// non-fatal shootings.
    static ref SOCRATA_VICTIMS_ENDPOINT: &'static str = "https://data.cityofchicago.org/resource/gumc-mgzr.json";
    /// The City of Chicago Socrata API endpoint for ShotSpotter alerts.
    static ref SOCRATA_SHOTSPOTTER_ENDPOINT: &'static str = "https://data.cityofchicago.org/resource/3h7q-7mdb.json";
}

/// The Chicago ShotSpotter map page.
#[function_component(Violence)]
pub fn violence() -> Html {
    background::set_background(true);
    gloo_utils::document().set_title("jl | violence");

    let is_loading = use_state(|| true);
    let get_chiraq_response = use_state(|| None);
    {
        let is_loading = is_loading.clone();
        let get_chiraq_response = get_chiraq_response.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                        "Violence in Chicago visualized".to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                        FAVICON_GIF.to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::PageType(PageType::Website))
                        .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::Title(
                        "jl | violence".to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));

                    match Request::get("/api/chiraq").send().await {
                        Ok(response) => match response.status() {
                            200 => {
                                is_loading.set(false);
                                response.json::<ChicagoMapData>().await.map_or_else(
                                    |error| {
                                        error!("FAILED TO PARSE CHIRAQ DATA TO THE CHICAGOMAPDATA STRUCT!");
                                        error!(error.to_string());

                                        is_loading.set(false);
                                        get_chiraq_response.set(Some(Err(
                                            Response::status_500_with_message(format!(
                                                "UNABLE TO PARSE CHIRAQ DATA TO JSON: {error}"
                                            )),
                                        )))
                                    },
                                    |shotspotter_data| {
                                        is_loading.set(false);
                                        get_chiraq_response.set(Some(Ok(shotspotter_data)))
                                    },
                                )
                            }
                            _ => {
                                error!(format!("{:?}", response));

                                is_loading.set(false);
                                get_chiraq_response.set(Some(Err(
                                    Response::status_500_with_message(
                                        "No API response".to_string(),
                                    ),
                                )));
                            }
                        },
                        Err(error) => {
                            error!(format!("{:?}", error));

                            is_loading.set(false);
                            get_chiraq_response.set(Some(Err(Response::status_500_with_message(
                                format!("UNABLE TO GET CHIRAQ DATA FROM THE API: {error}"),
                            ))));
                        }
                    }
                });
            },
            (),
        )
    }

    let chiraq_response = get_chiraq_response
        .as_ref()
        .unwrap_or(&Ok(ChicagoMapData::default()))
        .to_owned();

    let map = match chiraq_response {
        Ok(chicago_map_data) => {
            let shotspotter_data_array = chicago_map_data.shotspotter_data.as_array();
            let violence_data_array = chicago_map_data.violence_data.as_array();

            if let (Some(shotspotter_data), Some(violence_data)) =
                (shotspotter_data_array, violence_data_array)
            {
                if !shotspotter_data.is_empty() && !violence_data.is_empty() {
                    let map_html = render_map(chicago_map_data).unwrap_or_else(|error| {
                        // TODO: UPDATE THIS TO DISPLAY SOMETHING ELSE INSTEAD.
                        html! {
                            <div>
                              <h1>{ "FUCK!" }</h1>
                              <h2>{ "Shit done fucked up." }</h2>
                              <h3>{ format!("{error:#?}") }</h3>
                            </div>
                        }
                    });

                    map_html
                } else {
                    html! {
                        <Loading />
                    }
                }
            } else {
                // TODO: UPDATE THIS TO DISPLAY SOMETHING ELSE INSTEAD OF JUST THIS HEADER.
                html! {
                    <h1>{ "LOADING SHOTSPOTTER MAP..." }</h1>
                }
            }
        }
        Err(error) => {
            // TODO: UPDATE THIS TO DISPLAY SOMETHING ELSE INSTEAD.
            error!("CHICAGO_MAP_DATA IS ERROR!");
            error!(format!("{error:#?}"));

            html! {
                <div>
                  <h1>{ "FUCK!" }</h1>
                  <h2>{ "Shit done fucked up." }</h2>
                  <h3>{ format!("{error:#?}") }</h3>
                </div>
            }
        }
    };

    let page_view = html! {
        <div class="fade-in-slide-down">
          <div class="map-container component-container">
            { map }
          </div>
          <div>
            <small>
              <small>
                <small>
                  <blockquote>{ CHICAGO_DATA_DISCLAIMER.to_string() }</blockquote>
                  <blockquote>
                    <a href="https://www.flaticon.com/free-icons/shooting" title="shooting icons">
                      { "Shooting icons created by Freepik - Flaticon" }
                    </a>
                    { ", " }
                    <a href="https://www.flaticon.com/free-icons/bullet" title="bullet icons">
                        { "Bullet icons created by Nikita Golubev - Flaticon" }
                    </a>
                    { ", " }
                    <a href="https://www.flaticon.com/free-icons/affect" title="affect icons">
                      { "Affect icons created by IYIKON - Flaticon" }
                    </a>
                    { ", " }
                    <a href="https://www.flaticon.com/free-icons/dynamite" title="dynamite icons">
                        { "Dynamite icons created by Freepik - Flaticon" }
                    </a>
                  </blockquote>
                </small>
              </small>
            </small>
          </div>
          <div>
            { render_about_section() }
            { render_charts() }
          </div>
        </div>
    };

    utils::create_page_with_nav(
        None,
        if *is_loading {
            html! { <Loading /> }
        } else {
            // NOTE:
            // This block is necessary because the Leaflet container does not automatically detect
            // window resizing if the map container has an initial size state that eventually
            // changes, ie. the map is hidden at first, but is later revealed when an event occurs.
            // Leaflet tiles will only partially load until the user manually resizes the window. We
            // circumvent this by dispatching a resize `Event`, tricking Leaflet into recalculating
            // the tiles due to window resizing.
            let resize_event = Event::new("resize").ok();
            if let Some(resize_event) = resize_event {
                let _ = gloo_utils::window()
                    .dispatch_event(&resize_event)
                    .map_err(|error| {
                        error!("FAILED TO CALL RESIZE EVENT!");
                        error!(error);
                    });
            }

            page_view
        },
    )
}

/// Render the about this page section describing what's displayed here.
fn render_about_section() -> Html {
    html! {
        <div>
          <h3>{ "mapping chicago violence" }</h3>
          <p>{ "This map marks locations where Shotspotter alerts as well as victims of homicides and non-fatal shootings have been recorded." }</p>
        </div>
    }
}

/// Render miscellaneous charts using `Chart.js` that make the displayed data more digestable.
fn render_charts() -> Html {
    html! {
        <div>
          <p>{ "CHART DATA GOES HERE..." }</p>
          <p>{ "Display the following:" }</p>
          <p>{ "- Ranking of top 10 most common incidents" }</p>
          <p>{ "- Most common victims' race pie chart" }</p>
          <p>{ "- Most common sex pie chart" }</p>
          <p>{ "- Gunshot injury (yes or no) pie chart" }</p>
          <p>{ "- Top 10 most common community areas/zip codes pie chart" }</p>
        </div>
    }
}

/// Render the Shotspotter and violence map via Leaflet.
fn render_map(chicago_map_data: ChicagoMapData) -> Result<VNode, Html> {
    match gloo_utils::document().create_element("div") {
        Ok(map_container) => match map_container.dyn_into::<HtmlElement>() {
            Ok(container) => {
                container.set_class_name("map");

                let map = create_map(&container);

                if let (Some(shotspotter_data), Some(vhnfs_data)) = (
                    chicago_map_data.shotspotter_data.as_array(),
                    chicago_map_data.violence_data.as_array(),
                ) {
                    if !shotspotter_data.is_empty() {
                        let cleaned_shot_data = plot_shotspotter_data(&map, shotspotter_data)?;
                        // TODO: PLOT THE CLEANED SHOT DATA IN TABLES AND PIE CHARTS
                    }

                    if !vhnfs_data.is_empty() {
                        let cleaned_violence_data = plot_violence_data(&map, vhnfs_data)?;
                        // TODO: PLOT THE CLEANED SHOT DATA IN TABLES AND PIE CHARTS
                    }
                } else {
                    error!("FAILED TO GET SHOTSPOTTER AND VHNFS DATA FROM SHOTSPOTTER_DATA STRUCT");

                    // TODO: RETURN SOME SORT OF ERROR HTML HERE INSTEAD OF JUST LOGGING THE ERROR.
                }

                let node: &Node = &container.clone().into();
                let map = Html::VRef(node.clone());

                Ok(map)
            }
            Err(error) => {
                error!("FAILED TO CREATE THE MAP CONTAINER ELEMENT!");
                error!(&error);

                Err(html! {
                    <div>
                      <h1>{ "FUCK" }</h1>
                      <p>{ format!("{error:?}") }</p>
                    </div>
                })
            }
        },
        Err(error) => {
            error!("FAILED TO CREATE NEW DIV ELEMENT!");
            error!(&error);

            Err(html! {
                <div>
                  <h1>{ "FUCK" }</h1>
                  <p>{ format!("{error:?}") }</p>
                </div>
            })
        }
    }
}

/// Create the map and set its attribution layer.
fn create_map(container: &HtmlElement) -> Map {
    let map = Map::new_with_element(container, &MapOptions::default());
    map.set_view(&LatLng::new(41.87708716842721, -87.62622819781514), 11.0);

    let tile_layer_options = TileLayerOptions::new();
    tile_layer_options.set_attribution(JAWG_MAPS_ATTRIBUTION.to_string());

    TileLayer::new_options(&LEAFLET_URL_TEMPLATE, &tile_layer_options).add_to(&map);

    map
}

/// Plot Shotspotter markers and their corresponding popups on the Leaflet map.
fn plot_shotspotter_data(
    map: &Map,
    shotspotter_data: &Vec<Value>,
) -> Result<CleanedShotData, StaccError> {
    let shotspotter_layer = LayerGroup::new();

    let mut earliest_date = "".to_string();
    let mut latest_date = "".to_string();

    let mut cleaned_shot_data = CleanedShotData::new();

    for shot in shotspotter_data.into_iter() {
        let shot_data: Option<ShotData> = serde_json::from_value(shot.clone()).ok();

        if let Some(shot_data) = shot_data {
            let date = format_date(&shot_data.date);

            if earliest_date.is_empty() {
                earliest_date = date.clone();
            } else if date < earliest_date {
                earliest_date = date.clone();
            }

            if latest_date.is_empty() {
                latest_date = date.clone();
            } else if date > latest_date {
                latest_date = date.clone();
            }

            if let (Some(longitude), Some(latitude)) = (
                shot_data.location.coordinates.first(),
                shot_data.location.coordinates.last(),
            ) {
                let marker_icon =
                    if shot_data.incident_type_description.to_lowercase() == "multiple gunshots" {
                        MULTIPLE_SHOTSPOTTER_MARKER_ICON.to_string()
                    } else if shot_data.incident_type_description.to_lowercase()
                        == "gunshot or firecracker"
                    {
                        GUNSHOT_OR_FIRECRACKER_ICON.to_string()
                    } else {
                        SINGLE_SHOTSPOTTER_MARKER_ICON.to_string()
                    };
                let icon = create_map_marker_icon(marker_icon);
                let shot_marker = create_map_marker(
                    "🔫".to_string(),
                    &icon,
                    latitude,
                    longitude,
                    "shots fired".to_string(),
                );

                shot_marker.add_to_layer_group(&shotspotter_layer);

                if let Ok(popup_content) = shot_data.into_popup() {
                    let popup = create_marker_popup(&popup_content);
                    shot_marker.bind_popup(&popup);
                }
            }

            cleaned_shot_data
                .insert_or_increment("sorted_blocks", &shot_data.block.trim_end_matches(','))?;
            cleaned_shot_data
                .insert_or_increment("sorted_community_areas", &shot_data.community_area)?;
            cleaned_shot_data.insert_or_increment("sorted_dates", &date)?;
            cleaned_shot_data.insert_or_increment(
                "sorted_incident_types",
                &shot_data.incident_type_description,
            )?;
            cleaned_shot_data.insert_or_increment("sorted_rounds", &shot_data.rounds)?;
            cleaned_shot_data.insert_or_increment("sorted_zip_codes", &shot_data.zip_code)?;
        }
    }

    shotspotter_layer.add_to(map);

    cleaned_shot_data.time_range = (earliest_date, latest_date);

    Ok(cleaned_shot_data)
}

/// Plot violence markers and their corresponding popups on the Leaflet map.
fn plot_violence_data(
    map: &Map,
    vhnfs_data: &Vec<Value>,
) -> Result<CleanedViolenceData, StaccError> {
    let violence_layer = LayerGroup::new();

    let mut earliest_date = "".to_string();
    let mut latest_date = "".to_string();

    let mut cleaned_violence_data = CleanedViolenceData::new();

    for violence in vhnfs_data.into_iter() {
        let violence_data: Option<ViolenceData> = serde_json::from_value(violence.clone()).ok();

        if let Some(violence_data) = violence_data {
            let date = format_date(&violence_data.date);

            if earliest_date.is_empty() {
                earliest_date = date.clone();
            } else if date < earliest_date {
                earliest_date = date.clone();
            }

            if latest_date.is_empty() {
                latest_date = date.clone();
            } else if date > latest_date {
                latest_date = date.clone();
            }

            if let (Some(longitude), Some(latitude)) = (
                violence_data.location.coordinates.first(),
                violence_data.location.coordinates.last(),
            ) {
                let icon = create_map_marker_icon(VIOLENCE_MARKER_ICON.to_string());
                let violence_marker = create_map_marker(
                    "🤬".to_string(),
                    &icon,
                    latitude,
                    longitude,
                    "violence".to_string(),
                );

                violence_marker.add_to_layer_group(&violence_layer);

                if let Ok(popup_content) = violence_data.into_popup() {
                    let popup = create_marker_popup(&popup_content);
                    violence_marker.bind_popup(&popup);
                }
            }

            cleaned_violence_data.insert_or_increment("sorted_ages", &violence_data.age)?;
            cleaned_violence_data
                .insert_or_increment("sorted_community_areas", &violence_data.community_area)?;
            cleaned_violence_data.insert_or_increment("sorted_dates", &date)?;
            cleaned_violence_data
                .insert_or_increment("sorted_gun_injury_count", &violence_data.gunshot_injury_i)?;
            cleaned_violence_data.insert_or_increment(
                "sorted_incident_types",
                &violence_data.get_crime_description(),
            )?;
            cleaned_violence_data.insert_or_increment(
                "sorted_location_descriptions",
                &violence_data.location_description,
            )?;
            cleaned_violence_data
                .insert_or_increment("sorted_victim_races", &violence_data.race)?;
            cleaned_violence_data.insert_or_increment("sorted_victim_sexes", &violence_data.sex)?;
            cleaned_violence_data
                .insert_or_increment("sorted_zip_codes", &violence_data.zip_code)?;
        }
    }

    violence_layer.add_to(&map);

    cleaned_violence_data.time_range = (earliest_date, latest_date);

    Ok(cleaned_violence_data)
}

/// Create an icon for the map marker.
fn create_map_marker_icon(icon_url: String) -> Icon {
    let icon_options = IconOptions::new();
    icon_options.set_icon_url(icon_url);
    icon_options.set_icon_size(Point::new(40.0, 40.0));

    let icon = Icon::new(&icon_options);

    icon
}

/// Create a map marker from the given `Icon`, alternate icon, and coordinates.
fn create_map_marker(
    alt_icon: String,
    icon: &Icon,
    latitude: &f64,
    longitude: &f64,
    title: String,
) -> Marker {
    let marker_options = MarkerOptions::new();
    marker_options.set_alt(alt_icon);
    marker_options.set_icon(icon.clone());
    marker_options.set_title(title);

    let map_marker = Marker::new_with_options(&LatLng::new(*latitude, *longitude), &marker_options);

    map_marker
}

/// Create the popup for the map marker.
fn create_marker_popup(popup_content: &JsValue) -> Popup {
    let popup_options = PopupOptions::new();
    popup_options.set_class_name("marker-popup".to_string());
    popup_options.set_close_on_escape_key(true);
    popup_options.set_keep_in_view(true);

    let popup = Popup::new(&popup_options, None);
    popup.set_content(popup_content);

    popup
}
