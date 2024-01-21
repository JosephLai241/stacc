//! Contains structs for deserializing responses returned from the Chicago API.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Holds the Chicago ShotSpotter Alert and Victims of Homicide and Non-Fatal Shootings data.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChicagoMapData {
    /// Data returned from the Shotspotter Alert API endpoint.
    pub shotspotter_data: Value,
    /// Data returned from the Victims of Homicides and Non-Fatal Shootings API endpoint.
    pub violence_data: Value,
}

impl Default for ChicagoMapData {
    fn default() -> Self {
        Self {
            shotspotter_data: Value::Array(vec![]),
            violence_data: Value::Array(vec![]),
        }
    }
}

/// Contains cleaned/summarized data returned from the Shotspotter API.
#[derive(Debug)]
pub struct CleanedShotData {
    /// Most to least common blocks on which shots were detected (`ShotData.block`).
    pub sorted_blocks: HashMap<String, i32>,
    /// Most to least common community areas in which shots were detected (`ShotData.community_area`).
    pub sorted_community_areas: HashMap<String, i32>,
    /// Most to least common dates of occurrence (formatted date extracted from `ShotData.date`).
    pub sorted_dates: HashMap<String, i32>,
    /// Most to least common incident types (`ShotData.incident_type_description`).
    pub sorted_incident_types: HashMap<String, i32>,
    /// Most to least common number of rounds fired (`ShotData.rounds`).
    pub sorted_rounds: HashMap<String, i32>,
    /// Most to least common zip codes in which shots were detected (`ShotData.zip_code`).
    pub sorted_zip_codes: HashMap<String, i32>,
    /// The earliest to latest dates that have been plotted.
    pub time_range: (String, String),
}

impl CleanedShotData {
    /// Create a new instance of `CleanedShotData`.
    pub fn new() -> Self {
        Self {
            sorted_blocks: HashMap::new(),
            sorted_community_areas: HashMap::new(),
            sorted_dates: HashMap::new(),
            sorted_incident_types: HashMap::new(),
            sorted_rounds: HashMap::new(),
            sorted_zip_codes: HashMap::new(),
            time_range: ("".to_string(), "".to_string()),
        }
    }
}

/// Contains cleaned/summarized data returned from the violence data.
#[derive(Debug)]
pub struct CleanedViolenceData {
    /// Most to least common age ranges (`ViolenceData.age`).
    pub sorted_ages: HashMap<String, i32>,
    /// Most to least common community areas (`ViolenceData.community_area`).
    pub sorted_community_areas: HashMap<String, i32>,
    /// Most to least common dates of occurrence (formatted date extracted from
    /// `ViolenceData.community_area`).
    pub sorted_dates: HashMap<String, i32>,
    /// Count of yes or no gun injuries (`ViolenceData.gunshot_injury_i`).
    pub sorted_gun_injury_count: HashMap<String, i32>,
    /// Most to least common incident types. The keys correspond to the value returned from the
    /// match statement which matches the UCR code to its description
    /// (`ViolenceData.incident_iucr_cd`).
    pub sorted_incident_types: HashMap<String, i32>,
    /// Most to least common location descriptions (`ViolenceData.location_description`).
    pub sorted_location_descriptions: HashMap<String, i32>,
    /// Most to least common victim races (`ViolenceData.race`).
    pub sorted_victim_races: HashMap<String, i32>,
    /// Most to least common victim sexes (`ViolenceData.sex`).
    pub sorted_victim_sexes: HashMap<String, i32>,
    /// Most to least common zip codes (`ViolenceData.zip_code`).
    pub sorted_zip_codes: HashMap<String, i32>,
    /// The earliest to latest dates that have been plotted.
    pub time_range: (String, String),
}

impl CleanedViolenceData {
    /// Create a new instance of `CleanedViolenceData`.
    pub fn new() -> Self {
        Self {
            sorted_ages: HashMap::new(),
            sorted_community_areas: HashMap::new(),
            sorted_dates: HashMap::new(),
            sorted_gun_injury_count: HashMap::new(),
            sorted_incident_types: HashMap::new(),
            sorted_location_descriptions: HashMap::new(),
            sorted_victim_races: HashMap::new(),
            sorted_victim_sexes: HashMap::new(),
            sorted_zip_codes: HashMap::new(),
            time_range: ("".to_string(), "".to_string()),
        }
    }
}

/// Holds the fields contained in the `location` key.
#[derive(Debug, Deserialize)]
pub struct Location {
    /// The coordinates (longitude followed by latitude) at which the event occurred.
    pub coordinates: Vec<f64>,
}

/// Holds the most interesting data from the Shotspotter data.
/// Information for these fields reference the Socrata API documentation here:
/// [Socrata Shotspotter API Documentation](https://dev.socrata.com/foundry/data.cityofchicago.org/3h7q-7mdb)
#[derive(Debug, Deserialize)]
pub struct ShotData {
    /// The block on which the shots occurred.
    pub block: String,
    /// The name of the community in which shots were fired.
    pub community_area: String,
    /// The date (sometimes a rough estimation) on which the shots were fired.
    pub date: String,
    /// A short description of the incident.
    /// Alert types are “Single Gunshot,” “Multiple Gunshot,” and “Gunshot or Firecracker.”
    pub incident_type_description: String,
    /// The location of the shooting.
    pub location: Location,
    /// The number of shots/rounds detected.
    pub rounds: String,
    /// The zip code in which the shots occurred.
    pub zip_code: String,
}

/// Holds the most interesting data from the violence data.
/// Information for these fields reference the Socrata API documentation here:
/// [Socrata Violence API Documentation](https://dev.socrata.com/foundry/data.cityofchicago.org/gumc-mgzr)
#[derive(Debug, Deserialize)]
pub struct ViolenceData {
    /// The age range of the victim (ie. `0-19`, `20-29`, `30-39`).
    pub age: String,
    /// The name of the community in which the violence occurred (ie. `AUBURN GRESHAM`, `AUSTIN`,
    /// `BELMONT CRAGIN`).
    pub community_area: String,
    /// The date (sometimes a rough estimation) on which the violence occurred.
    pub date: String,
    /// Whether there was a gunshot injury ("YES" or "NO").
    pub gunshot_injury_i: String,
    /// Based on the Illinois Uniform Crime Reporting code. This is directly linked to the Primary
    /// Type and Description.
    pub incident_iucr_cd: String,
    /// The name of the incident.
    pub incident_primary: String,
    /// The location of the shooting.
    pub location: Location,
    /// A short description of the location at which the violence occurred.
    pub location_description: String,
    /// An abbreviation of the race of the victim (ie. `API`, `BLK`, `WHI`).
    pub race: String,
    /// A single letter denoting the sex of the victim (ie. `F`, `M`).
    pub sex: String,
    /// Crime classification as outlined in the FBI's Uniform Crime Reporting (UCR). See the Chicago
    /// Police Department listing of these classifications at: http://gis.chicagopolice.org/clearmapcrimesums/crime_type.
    pub victimization_fbi_cd: String,
    /// The FBI's text description of the incident.
    /// FBI Description connects a text description of the category to FBI Code.
    pub victimization_fbi_descr: String,
    /// The zip code in which the violence occurred.
    pub zip_code: String,
}

impl ViolenceData {
    /// Get the crime description that matches the `self.incident_iucr_cd` code.
    /// Codes were found at this [Chicago Police Crime Details page](https://gis.chicagopolice.org/pages/crime_details).
    ///
    /// Holy fucking shit Vim/Neovim's macros saved me so much fucking time. 🅱️esus 🅱️UCCing 🅱️hrist
    /// Neovim is the best.
    pub fn get_crime_description(&self) -> String {
        match self.incident_iucr_cd.as_str() {
            "0110" => "HOMICIDE FIRST DEGREE MURDER".to_string(),
            "0130" => "HOMICIDE SECOND DEGREE MURDER".to_string(),
            "0141" => "HOMICIDE INVOLUNTARY MANSLAUGHTER".to_string(),
            "0142" => "HOMICIDE RECKLESS HOMICIDE".to_string(),
            "0261" => "CRIM SEXUAL ASSAULT AGGRAVATED: HANDGUN".to_string(),
            "0262" => "CRIM SEXUAL ASSAULT AGGRAVATED: OTHER FIREARM".to_string(),
            "0263" => "CRIM SEXUAL ASSAULT AGGRAVATED: KNIFE/CUT INSTR".to_string(),
            "0264" => "CRIM SEXUAL ASSAULT AGGRAVATED: OTHER DANG WEAPON".to_string(),
            "0265" => "CRIM SEXUAL ASSAULT AGGRAVATED: OTHER".to_string(),
            "0266" => "CRIM SEXUAL ASSAULT PREDATORY".to_string(),
            "0271" => "CRIM SEXUAL ASSAULT ATTEMPT AGG: HANDGUN".to_string(),
            "0272" => "CRIM SEXUAL ASSAULT ATTEMPT AGG: OTHER FIREARM".to_string(),
            "0273" => "CRIM SEXUAL ASSAULT ATTEMPT AGG: KNIFE/CUT INSTR".to_string(),
            "0274" => "CRIM SEXUAL ASSAULT ATTEMPT AGG: OTHER DANG WEAPON".to_string(),
            "0275" => "CRIM SEXUAL ASSAULT ATTEMPT AGG: OTHER".to_string(),
            "0281" => "CRIM SEXUAL ASSAULT NON-AGGRAVATED".to_string(),
            "0291" => "CRIM SEXUAL ASSAULT ATTEMPT NON-AGGRAVATED".to_string(),
            "1753" => "OFFENSE INVOLVING CHILDREN SEX ASSLT OF CHILD BY FAM MBR".to_string(),
            "1754" => "OFFENSE INVOLVING CHILDREN AGG SEX ASSLT OF CHILD FAM MBR".to_string(),
            "0312" => "ROBBERY ARMED:KNIFE/CUTTING INSTRUMENT".to_string(),
            "0313" => "ROBBERY ARMED: OTHER DANGEROUS WEAPON".to_string(),
            "031A" => "ROBBERY ARMED: HANDGUN".to_string(),
            "031B" => "ROBBERY ARMED: OTHER FIREARM".to_string(),
            "0320" => "ROBBERY STRONGARM - NO WEAPON".to_string(),
            "0325" => "ROBBERY VEHICULAR HIJACKING".to_string(),
            "0326" => "ROBBERY AGGRAVATED VEHICULAR HIJACKING".to_string(),
            "0330" => "ROBBERY AGGRAVATED".to_string(),
            "0331" => "ROBBERY ATTEMPT: AGGRAVATED".to_string(),
            "0334" => "ROBBERY ATTEMPT: ARMED-KNIFE/CUT INSTR".to_string(),
            "0337" => "ROBBERY ATTEMPT: ARMED-OTHER DANG WEAP".to_string(),
            "033A" => "ROBBERY ATTEMPT: ARMED-HANDGUN".to_string(),
            "033B" => "ROBBERY ATTEMPT: ARMED-OTHER FIREARM".to_string(),
            "0340" => "ROBBERY ATTEMPT: STRONGARM-NO WEAPON".to_string(),
            "051A" => "ASSAULT AGGRAVATED: HANDGUN".to_string(),
            "051B" => "ASSAULT AGGRAVATED: OTHER FIREARM".to_string(),
            "0520" => "ASSAULT AGGRAVATED:KNIFE/CUTTING INSTR".to_string(),
            "0530" => "ASSAULT AGGRAVATED: OTHER DANG WEAPON".to_string(),
            "0550" => "ASSAULT AGGRAVATED PO: HANDGUN".to_string(),
            "0551" => "ASSAULT AGGRAVATED PO: OTHER FIREARM".to_string(),
            "0552" => "ASSAULT AGGRAVATED PO:KNIFE/CUT INSTR".to_string(),
            "0553" => "ASSAULT AGGRAVATED PO: OTHER DANG WEAP".to_string(),
            "0555" => "ASSAULT AGG PRO.EMP: HANDGUN".to_string(),
            "0556" => "ASSAULT AGG PRO.EMP: OTHER FIREARM".to_string(),
            "0557" => "ASSAULT AGG PRO.EMP:KNIFE/CUTTING INST".to_string(),
            "0558" => "ASSAULT AGG PRO.EMP: OTHER DANG WEAPON".to_string(),
            "041A" => "BATTERY AGGRAVATED: HANDGUN".to_string(),
            "041B" => "BATTERY AGGRAVATED: OTHER FIREARM".to_string(),
            "0420" => "BATTERY AGGRAVATED:KNIFE/CUTTING INSTR".to_string(),
            "0430" => "BATTERY AGGRAVATED: OTHER DANG WEAPON".to_string(),
            "0450" => "BATTERY AGGRAVATED PO: HANDGUN".to_string(),
            "0451" => "BATTERY AGGRAVATED PO: OTHER FIREARM".to_string(),
            "0452" => "BATTERY AGGRAVATED PO: KNIFE/CUT INSTR".to_string(),
            "0453" => "BATTERY AGGRAVATED PO: OTHER DANG WEAP".to_string(),
            "0461" => "BATTERY AGG PO HANDS ETC SERIOUS INJ".to_string(),
            "0462" => "BATTERY AGG PRO EMP HANDS SERIOUS INJ".to_string(),
            "0479" => "BATTERY AGG: HANDS/FIST/FEET SERIOUS INJURY".to_string(),
            "0480" => "BATTERY AGG PRO.EMP: HANDGUN".to_string(),
            "0481" => "BATTERY AGG PRO.EMP: OTHER FIREARM".to_string(),
            "0482" => "BATTERY AGG PRO.EMP:KNIFE/CUTTING INST".to_string(),
            "0483" => "BATTERY AGG PRO.EMP: OTHER DANG WEAPON".to_string(),
            "0485" => "BATTERY AGGRAVATED OF A CHILD".to_string(),
            "0488" => "BATTERY AGGRAVATED DOMESTIC BATTERY: HANDGUN".to_string(),
            "0489" => "BATTERY AGGRAVATED DOMESTIC BATTERY: OTHER FIREARM".to_string(),
            "0490" => "RITUALISM AGG RITUAL MUT:HANDGUN".to_string(),
            "0491" => "RITUALISM AGG RITUAL MUT:OTHER FIREARM".to_string(),
            "0492" => "RITUALISM AGG RITUAL MUT:KNIFE/CUTTING I".to_string(),
            "0493" => "RITUALISM AGG RITUAL MUT:OTH DANG WEAPON".to_string(),
            "0495" => "BATTERY AGGRAVATED OF A SENIOR CITIZEN".to_string(),
            "0496" => "BATTERY AGGRAVATED DOMESTIC BATTERY: KNIFE/CUTTING INST".to_string(),
            "0497" => "BATTERY AGGRAVATED DOMESTIC BATTERY: OTHER DANG WEAPON".to_string(),
            "0498" => {
                "BATTERY AGGRAVATED DOMESTIC BATTERY: HANDS/FIST/FEET SERIOUS INJURY".to_string()
            }
            "0510" => "RITUALISM AGG RIT MUT: HANDS/FIST/FEET SERIOUS INJURY".to_string(),
            "0610" => "BURGLARY FORCIBLE ENTRY".to_string(),
            "0620" => "BURGLARY UNLAWFUL ENTRY".to_string(),
            "0630" => "BURGLARY ATTEMPT FORCIBLE ENTRY".to_string(),
            "0650" => "BURGLARY HOME INVASION".to_string(),
            "0810" => "THEFT OVER $300".to_string(),
            "0820" => "THEFT $300 AND UNDER".to_string(),
            "0840" => "THEFT FINANCIAL ID THEFT: OVER $300".to_string(),
            "0841" => "THEFT FINANCIAL ID THEFT:$300 &UNDER".to_string(),
            "0842" => "THEFT AGG: FINANCIAL ID THEFT".to_string(),
            "0843" => "THEFT ATTEMPT FINANCIAL IDENTITY THEFT".to_string(),
            "0850" => "THEFT ATTEMPT THEFT".to_string(),
            "0860" => "THEFT RETAIL THEFT".to_string(),
            "0865" => "THEFT DELIVERY CONTAINER THEFT".to_string(),
            "0870" => "THEFT POCKET-PICKING".to_string(),
            "0880" => "THEFT PURSE-SNATCHING".to_string(),
            "0890" => "THEFT FROM BUILDING".to_string(),
            "0895" => "THEFT FROM COIN-OP MACHINE/DEVICE".to_string(),
            "0910" => "MOTOR VEHICLE THEFT AUTOMOBILE".to_string(),
            "0915" => "MOTOR VEHICLE THEFT TRUCK, BUS, MOTOR HOME".to_string(),
            "0917" => "MOTOR VEHICLE THEFT CYCLE, SCOOTER, BIKE W-VIN".to_string(),
            "0918" => "MOTOR VEHICLE THEFT CYCLE, SCOOTER, BIKE NO VIN".to_string(),
            "0920" => "MOTOR VEHICLE THEFT ATT: AUTOMOBILE".to_string(),
            "0925" => "MOTOR VEHICLE THEFT ATT: TRUCK, BUS, MOTOR HOME".to_string(),
            "0927" => "MOTOR VEHICLE THEFT ATTEMPT: CYCLE, SCOOTER, BIKE W-VIN".to_string(),
            "0928" => "MOTOR VEHICLE THEFT ATTEMPT: CYCLE, SCOOTER, BIKE NO VIN".to_string(),
            "0930" => "MOTOR VEHICLE THEFT THEFT/RECOVERY: AUTOMOBILE".to_string(),
            "0935" => "MOTOR VEHICLE THEFT THEFT/RECOVERY: TRUCK,BUS,MHOME".to_string(),
            "0937" => "MOTOR VEHICLE THEFT THEFT/RECOVERY: CYCLE, SCOOTER, BIKE W-VIN".to_string(),
            "0938" => "MOTOR VEHICLE THEFT THEFT/RECOVERY: CYCLE, SCOOTER, BIKE NO VIN".to_string(),
            "0545" => "ASSAULT PRO EMP HANDS NO/MIN INJURY".to_string(),
            "0554" => "ASSAULT AGG PO HANDS NO/MIN INJURY".to_string(),
            "0560" => "ASSAULT SIMPLE".to_string(),
            "0580" => "STALKING SIMPLE".to_string(),
            "0581" => "STALKING AGGRAVATED".to_string(),
            "0583" => "STALKING CYBERSTALKING".to_string(),
            "0440" => "BATTERY AGG: HANDS/FIST/FEET NO/MINOR INJURY".to_string(),
            "0454" => "BATTERY AGG PO HANDS NO/MIN INJURY".to_string(),
            "0460" => "BATTERY SIMPLE".to_string(),
            "0475" => "BATTERY OF UNBORN CHILD".to_string(),
            "0484" => "BATTERY PRO EMP HANDS NO/MIN INJURY".to_string(),
            "0486" => "BATTERY DOMESTIC BATTERY SIMPLE".to_string(),
            "0487" => "BATTERY AGGRAVATED OF A UNBORN CHILD".to_string(),
            "0494" => "RITUALISM AGG RIT MUT: HANDS/FIST/FEET NO/MINOR INJURY".to_string(),
            "1010" => "ARSON BY EXPLOSIVE".to_string(),
            "1020" => "ARSON BY FIRE".to_string(),
            "1025" => "ARSON AGGRAVATED".to_string(),
            "1090" => "ARSON ATTEMPT ARSON".to_string(),
            "1120" => "DECEPTIVE PRACTICE FORGERY".to_string(),
            "1121" => "DECEPTIVE PRACTICE COUNTERFEITING DOCUMENT".to_string(),
            "1122" => "DECEPTIVE PRACTICE COUNTERFEIT CHECK".to_string(),
            "1110" => "DECEPTIVE PRACTICE BOGUS CHECK".to_string(),
            "1130" => "DECEPTIVE PRACTICE FRAUD OR CONFIDENCE GAME".to_string(),
            "1135" => "DECEPTIVE PRACTICE INSURANCE FRAUD".to_string(),
            "1150" => "DECEPTIVE PRACTICE CREDIT CARD FRAUD".to_string(),
            "1151" => "DECEPTIVE PRACTICE ILLEGAL POSSESSION CASH CARD".to_string(),
            "1152" => "DECEPTIVE PRACTICE ILLEGAL USE CASH CARD".to_string(),
            "1160" => "DECEPTIVE PRACTICE ALTER COINS".to_string(),
            "1170" => "DECEPTIVE PRACTICE IMPERSONATION".to_string(),
            "1185" => "DECEPTIVE PRACTICE DECEPTIVE COLLECTION PRACTICES".to_string(),
            "1195" => "DECEPTIVE PRACTICE FINAN EXPLOIT-ELDERLY/DISABLED".to_string(),
            "1205" => "DECEPTIVE PRACTICE THEFT BY LESSEE,NON-VEH".to_string(),
            "1206" => "DECEPTIVE PRACTICE THEFT BY LESSEE,MOTOR VEH".to_string(),
            "1210" => "DECEPTIVE PRACTICE THEFT OF LABOR/SERVICES".to_string(),
            "1220" => "DECEPTIVE PRACTICE THEFT OF LOST/MISLAID PROP".to_string(),
            "1230" => "DECEPTIVE PRACTICE POSS. KEYS OR DEV.TO COIN MACH".to_string(),
            "1235" => "DECEPTIVE PRACTICE UNLAWFUL USE OF RECORDED SOUND".to_string(),
            "1240" => "DECEPTIVE PRACTICE UNLAWFUL USE OF A COMPUTER".to_string(),
            "1241" => "DECEPTIVE PRACTICE AGGRAVATED COMPUTER TAMPERING".to_string(),
            "1242" => "DECEPTIVE PRACTICE COMPUTER FRAUD".to_string(),
            "1245" => "DECEPTIVE PRACTICE PAY TV SERVICE OFFENSES".to_string(),
            "1255" => "DECEPTIVE PRACTICE UNIDENTIFIABLE RECORDING SOUND".to_string(),
            "1260" => "DECEPTIVE PRACTICE LIBRARY THEFT".to_string(),
            "1261" => "DECEPTIVE PRACTICE UNAUTHORIZED VIDEOTAPING".to_string(),
            "1265" => "CRIMINAL DAMAGE LIBRARY VANDALISM".to_string(),
            "1305" => "CRIMINAL DAMAGE CRIMINAL DEFACEMENT".to_string(),
            "1140" => "DECEPTIVE PRACTICE EMBEZZLEMENT".to_string(),
            "1200" => "DECEPTIVE PRACTICE STOLEN PROP: BUY/RECEIVE/POS.".to_string(),
            "1310" => "CRIMINAL DAMAGE TO PROPERTY".to_string(),
            "1320" => "CRIMINAL DAMAGE TO VEHICLE".to_string(),
            "1340" => "CRIMINAL DAMAGE TO STATE SUP PROP".to_string(),
            "1345" => "CRIMINAL DAMAGE TO CITY OF CHICAGO PROPERTY".to_string(),
            "1370" => "CRIMINAL DAMAGE TO FIRE FIGHT.APP.EQUIP".to_string(),
            "1375" => "CRIMINAL DAMAGE INSTITUTIONAL VANDALISM".to_string(),
            "141A" => "WEAPONS VIOLATION UNLAWFUL USE HANDGUN".to_string(),
            "141B" => "WEAPONS VIOLATION UNLAWFUL USE OTHER FIREARM".to_string(),
            "141C" => "WEAPONS VIOLATION UNLAWFUL USE OTHER DANG WEAPON".to_string(),
            "142A" => "WEAPONS VIOLATION UNLAWFUL SALE HANDGUN".to_string(),
            "142B" => "WEAPONS VIOLATION UNLAWFUL SALE OTHER FIREARM".to_string(),
            "1435" => "WEAPONS VIOLATION POS: FIREARM AT SCHOOL".to_string(),
            "143A" => "WEAPONS VIOLATION UNLAWFUL POSS OF HANDGUN".to_string(),
            "143B" => "WEAPONS VIOLATION UNLAWFUL POSS OTHER FIREARM".to_string(),
            "143C" => "WEAPONS VIOLATION UNLAWFUL POSS AMMUNITION".to_string(),
            "1440" => "WEAPONS VIOLATION REGISTER OF SALES BY DEALER".to_string(),
            "1450" => "WEAPONS VIOLATION DEFACE IDENT MARKS OF FIREARM".to_string(),
            "1460" => "WEAPONS VIOLATION POSS FIREARM/AMMO:NO FOID CARD".to_string(),
            "1475" => "WEAPONS VIOLATION SALE OF METAL PIERCING BULLETS".to_string(),
            "1476" => "WEAPONS VIOLATION USE OF METAL PIERCING BULLETS".to_string(),
            "1477" => "WEAPONS VIOLATION RECKLESS FIREARM DISCHARGE".to_string(),
            "2900" => "WEAPONS VIOLATION UNLAWFUL USE/SALE AIR RIFLE".to_string(),
            "1505" => "PROSTITUTION CALL OPERATION".to_string(),
            "1506" => "PROSTITUTION SOLICIT ON PUBLIC WAY".to_string(),
            "1507" => "PROSTITUTION SOLICIT OFF PUBLIC WAY".to_string(),
            "1510" => "PROSTITUTION CAB OPERATION".to_string(),
            "1511" => "PROSTITUTION IN TAVERN".to_string(),
            "1512" => "PROSTITUTION SOLICIT FOR PROSTITUTE".to_string(),
            "1513" => "PROSTITUTION SOLICIT FOR BUSINESS".to_string(),
            "1515" => "PROSTITUTION PANDERING".to_string(),
            "1520" => "PROSTITUTION KEEP PLACE OF PROSTITUTION".to_string(),
            "1521" => "PROSTITUTION KEEP PLACE OF JUV PROSTITUTION".to_string(),
            "1525" => "PROSTITUTION PATRONIZING A PROSTITUTE".to_string(),
            "1526" => "PROSTITUTION PATRONIZE JUVENILE PROSTITUTE".to_string(),
            "1530" => "PROSTITUTION PIMPING".to_string(),
            "1531" => "PROSTITUTION JUVENILE PIMPING".to_string(),
            "1537" => "OFFENSE INVOLVING CHILDREN POS: PORNOGRAPHIC PRINT".to_string(),
            "1542" => "OBSCENITY SALE OF OBSCENE MATERIALS".to_string(),
            "1544" => "SEX OFFENSE SEXUAL EXPLOITATION OF A CHILD".to_string(),
            "1549" => "PROSTITUTION OTHER PROSTITUTION OFFENSE".to_string(),
            "1535" => "OBSCENITY OBSCENITY".to_string(),
            "1536" => "PUBLIC INDECENCY LICENSED PREMISE".to_string(),
            "1540" => "OBSCENITY OBSCENE MATTER".to_string(),
            "1541" => "OBSCENITY SALE/DIST OBSCENE MAT TO MINOR".to_string(),
            "1562" => "SEX OFFENSE AGG CRIMINAL SEXUAL ABUSE".to_string(),
            "1563" => "SEX OFFENSE CRIMINAL SEXUAL ABUSE".to_string(),
            "1564" => "SEX OFFENSE CRIMINAL TRANSMISSION OF HIV".to_string(),
            "1565" => "SEX OFFENSE INDECENT SOLICITATION/CHILD".to_string(),
            "1566" => "SEX OFFENSE INDECENT SOLICITATION/ADULT".to_string(),
            "1570" => "SEX OFFENSE PUBLIC INDECENCY".to_string(),
            "1572" => "SEX OFFENSE ADULTRY".to_string(),
            "1574" => "SEX OFFENSE FORNICATION".to_string(),
            "1576" => "SEX OFFENSE BIGAMY".to_string(),
            "1578" => "SEX OFFENSE MARRYING A BIGAMIST".to_string(),
            "1580" => "SEX OFFENSE SEX RELATION IN FAMILY".to_string(),
            "1582" => "OFFENSE INVOLVING CHILDREN CHILD PORNOGRAPHY".to_string(),
            "1585" => "SEX OFFENSE OTHER".to_string(),
            "1590" => "SEX OFFENSE ATT AGG CRIMINAL SEXUAL ABUSE".to_string(),
            "2830" => "OTHER OFFENSE OBSCENE TELEPHONE CALLS".to_string(),
            "5004" => "SEX OFFENSE ATT CRIM SEXUAL ABUSE".to_string(),
            "1811" => "NARCOTICS POSS: CANNABIS 30GMS OR LESS".to_string(),
            "1812" => "NARCOTICS POSS: CANNABIS MORE THAN 30GMS".to_string(),
            "1821" => "NARCOTICS MANU/DEL:CANNABIS 10GM OR LESS".to_string(),
            "1822" => "NARCOTICS MANU/DEL:CANNABIS OVER 10 GMS".to_string(),
            "1840" => "NARCOTICS DELIVER CANNABIS TO PERSON <18".to_string(),
            "1850" => "NARCOTICS CANNABIS PLANT".to_string(),
            "1860" => "NARCOTICS CALCULATED CANNABIS CONSPIRACY".to_string(),
            "1900" => "OTHER NARCOTIC VIOLATION INTOXICATING COMPOUNDS".to_string(),
            "2010" => "NARCOTICS MANU/DELIVER:AMPHETAMINES".to_string(),
            "2011" => "NARCOTICS MANU/DELIVER:BARBITUATES".to_string(),
            "2012" => "NARCOTICS MANU/DELIVER:COCAINE".to_string(),
            "2013" => "NARCOTICS MANU/DELIVER: HEROIN(BRN/TAN)".to_string(),
            "2014" => "NARCOTICS MANU/DELIVER: HEROIN (WHITE)".to_string(),
            "2015" => "NARCOTICS MANU/DELIVER: HALLUCINOGEN".to_string(),
            "2016" => "NARCOTICS MANU/DELIVER:PCP".to_string(),
            "2017" => "NARCOTICS MANU/DELIVER:CRACK".to_string(),
            "2018" => "NARCOTICS MANU/DELIVER:SYNTHETIC DRUGS".to_string(),
            "2019" => "NARCOTICS MANU/DELIVER:HEROIN(BLACK TAR)".to_string(),
            "2020" => "NARCOTICS POSS: AMPHETAMINES".to_string(),
            "2021" => "NARCOTICS POSS: BARBITUATES".to_string(),
            "2022" => "NARCOTICS POSS: COCAINE".to_string(),
            "2023" => "NARCOTICS POSS: HEROIN(BRN/TAN)".to_string(),
            "2024" => "NARCOTICS POSS: HEROIN(WHITE)".to_string(),
            "2025" => "NARCOTICS POSS: HALLUCINOGENS".to_string(),
            "2026" => "NARCOTICS POSS: PCP".to_string(),
            "2027" => "NARCOTICS POSS: CRACK".to_string(),
            "2028" => "NARCOTICS POSS: SYNTHETIC DRUGS".to_string(),
            "2029" => "NARCOTICS POSS: HEROIN(BLACK TAR)".to_string(),
            "2030" => "NARCOTICS MANU/DELIVER:LOOK-ALIKE DRUG".to_string(),
            "2031" => "NARCOTICS POSS: METHAMPHETAMINES".to_string(),
            "2032" => "NARCOTICS MANU/DELIVER: METHAMPHETAMINES".to_string(),
            "2040" => "NARCOTICS POSS: LOOK-ALIKE DRUGS".to_string(),
            "2050" => "NARCOTICS CRIMINAL DRUG CONSPIRACY".to_string(),
            "2060" => "NARCOTICS FAIL REGISTER LIC:CONT SUBS".to_string(),
            "2070" => "NARCOTICS DEL CONT SUBS TO PERSON <18".to_string(),
            "2080" => "NARCOTICS CONT SUBS:FAIL TO MAINT RECORD".to_string(),
            "2090" => "NARCOTICS ALTER/FORGE PRESCRIPTION".to_string(),
            "2094" => "NARCOTICS ATTEMPT POSSESSION CANNABIS".to_string(),
            "2095" => "NARCOTICS ATTEMPT POSSESSION NARCOTICS".to_string(),
            "2110" => "NARCOTICS POS: HYPODERMIC NEEDLE".to_string(),
            "2170" => "NARCOTICS POSSESSION OF DRUG EQUIPMENT".to_string(),
            "1610" => "GAMBLING BOOKMAKING/HORSES".to_string(),
            "1611" => "GAMBLING BOOKMAKING/SPORTS".to_string(),
            "1620" => "GAMBLING BOLITA OR BOLI PUL/OFFICE".to_string(),
            "1621" => "GAMBLING BOLITA OR BOLI PUL/RUNNER".to_string(),
            "1622" => "GAMBLING BOLITA OR BOLI PUL/WRITER".to_string(),
            "1623" => "GAMBLING BOLITA OR BOLI PUL/STATION".to_string(),
            "1624" => "GAMBLING LOTTERY/PARI-MUTUEL".to_string(),
            "1625" => "GAMBLING NATIONAL LOTTERY".to_string(),
            "1626" => "GAMBLING ILLEGAL ILL LOTTERY".to_string(),
            "1627" => "GAMBLING LOTTERY/OTHER".to_string(),
            "1630" => "GAMBLING WIREROOM/HORSES".to_string(),
            "1631" => "GAMBLING WIREROOM/SPORTS".to_string(),
            "1632" => "GAMBLING WIREROOM/NUMBERS".to_string(),
            "1633" => "GAMBLING SPORTS TAMPERING".to_string(),
            "1640" => "GAMBLING REGISTER FED GAMBLING STAMP".to_string(),
            "1650" => "GAMBLING VIOL CHARITABLE GAME ACT".to_string(),
            "1651" => "GAMBLING GAME/CARDS".to_string(),
            "1661" => "GAMBLING GAME/DICE".to_string(),
            "1670" => "GAMBLING GAME/AMUSEMENT DEVICE".to_string(),
            "1680" => "GAMBLING OTHER".to_string(),
            "1681" => "GAMBLING LOTTERY/PARLAY CARDS".to_string(),
            "1682" => "OTHER OFFENSE ANIMAL FIGHTING".to_string(),
            "1690" => "GAMBLING POLICY/HOUSEBOOK".to_string(),
            "1691" => "GAMBLING POLICY/STATION".to_string(),
            "1692" => "GAMBLING POLICY/RUNNER".to_string(),
            "1693" => "GAMBLING POLICY/TURN-IN".to_string(),
            "1694" => "GAMBLING POLICY/OFFICE".to_string(),
            "1695" => "GAMBLING POLICY/PRESS".to_string(),
            "1696" => "GAMBLING POLICY/WHEEL".to_string(),
            "1697" => "GAMBLING POLICY/OTHER".to_string(),
            "1720" => "OFFENSE INVOLVING CHILDREN CONTRIBUTE DELINQUENCY OF A CHILD".to_string(),
            "1750" => "OFFENSE INVOLVING CHILDREN CHILD ABUSE".to_string(),
            "1751" => "OFFENSE INVOLVING CHILDREN CRIM SEX ABUSE BY FAM MEMBER".to_string(),
            "1752" => "OFFENSE INVOLVING CHILDREN AGG CRIM SEX ABUSE FAM MEMBER".to_string(),
            "1790" => "OFFENSE INVOLVING CHILDREN CHILD ABDUCTION".to_string(),
            "1791" => "OFFENSE INVOLVING CHILDREN HARBOR RUNAWAY".to_string(),
            "1792" => "KIDNAPPING CHILD ABDUCTION/STRANGER".to_string(),
            "2210" => "LIQUOR LAW VIOLATION SELL/GIVE/DEL LIQUOR TO MINOR".to_string(),
            "2220" => "LIQUOR LAW VIOLATION ILLEGAL POSSESSION BY MINOR".to_string(),
            "2230" => "LIQUOR LAW VIOLATION ILLEGAL CONSUMPTION BY MINOR".to_string(),
            "2240" => "LIQUOR LAW VIOLATION MINOR MISREPRESENT AGE".to_string(),
            "2250" => "LIQUOR LAW VIOLATION LIQUOR LICENSE VIOLATION".to_string(),
            "2251" => "LIQUOR LAW VIOLATION EMPLOY MINOR".to_string(),
            "0470" => "PUBLIC PEACE VIOLATION RECKLESS CONDUCT".to_string(),
            "2840" => "PUBLIC PEACE VIOLATION FALSE FIRE ALARM".to_string(),
            "2860" => "PUBLIC PEACE VIOLATION FALSE POLICE REPORT".to_string(),
            "2870" => "PUBLIC PEACE VIOLATION PEEPING TOM".to_string(),
            "3100" => "PUBLIC PEACE VIOLATION MOB ACTION".to_string(),
            "3610" => "OTHER OFFENSE INTERFERE W/ HIGHER EDUCATION".to_string(),
            "3710" => "INTERFERE WITH PUBLIC OFFICER RESIST/OBSTRUCT/DISARM OFFICER".to_string(),
            "3720" => "INTERFERE WITH PUBLIC OFFICER REFUSING TO AID AN OFFICER".to_string(),
            "3730" => "INTERFERE WITH PUBLIC OFFICER OBSTRUCTING JUSTICE".to_string(),
            "3740" => "INTERFERE WITH PUBLIC OFFICER CONCEALING/AIDING A FUGITIVE".to_string(),
            "3750" => "INTERFERE WITH PUBLIC OFFICER ESCAPE".to_string(),
            "3751" => "INTERFERE WITH PUBLIC OFFICER AIDING ARRESTEE ESCAPE".to_string(),
            "3760" => "INTERFERE WITH PUBLIC OFFICER OBSTRUCTING SERVICE".to_string(),
            "1030" => "ARSON POS: EXPLOSIVE/INCENDIARY DEV".to_string(),
            "1035" => "ARSON POS: CHEMICAL/DRY-ICE DEVICE".to_string(),
            "1330" => "CRIMINAL TRESPASS TO LAND".to_string(),
            "1335" => "CRIMINAL TRESPASS TO AIRPORT".to_string(),
            "1350" => "CRIMINAL TRESPASS TO STATE SUP LAND".to_string(),
            "1360" => "CRIMINAL TRESPASS TO VEHICLE".to_string(),
            "1365" => "CRIMINAL TRESPASS TO RESIDENCE".to_string(),
            "1710" => "OFFENSE INVOLVING CHILDREN ENDANGERING LIFE/HEALTH CHILD".to_string(),
            "1715" => "OFFENSES INVOLVING CHILDREN SALE TOBACCO PRODUCTS TOMINOR".to_string(),
            "1725" => "OFFENSES INVOLVING CHILDREN CONTRIBUTE CRIM DELINQUENCYJUVENILE".to_string(),
            "1755" => "OFFENSES INVOLVING CHILDREN CHILD ABANDONMENT".to_string(),
            "1775" => "OFFENSES INVOLVING CHILDREN SALE OF TRAVEL TICKET TO MINOR".to_string(),
            "1780" => "OFFENSE INVOLVING CHILDREN OTHER OFFENSE".to_string(),
            "2091" => "NARCOTICS FORFEIT PROPERTY".to_string(),
            "2092" => "NARCOTICS SOLICIT NARCOTICS ON PUBLICWAY".to_string(),
            "2093" => "NARCOTICS FOUND SUSPECT NARCOTICS".to_string(),
            "2111" => "NARCOTICS SALE/DEL HYPODERMIC NEEDLE".to_string(),
            "2120" => "NARCOTICS FAILURE TO KEEP HYPO RECORDS".to_string(),
            "2160" => "NARCOTICS SALE/DEL DRUG PARAPHERNALIA".to_string(),
            "2500" => "CRIMINAL ABORTION CRIMINAL ABORTION".to_string(),
            "2820" => "OTHER OFFENSE TELEPHONE THREAT".to_string(),
            "2825" => "OTHER OFFENSE HARASSMENT BY TELEPHONE".to_string(),
            "2826" => "OTHER OFFENSE HARASSMENT BY ELECTRONIC MEANS".to_string(),
            "2850" => "PUBLIC PEACE VIOLATION BOMB THREAT".to_string(),
            "2851" => "PUBLIC PEACE VIOLATION ARSON THREAT".to_string(),
            "2890" => "PUBLIC PEACE VIOLATION OTHER VIOLATION".to_string(),
            "2895" => "PUBLIC PEACE VIOLATION INTERFERE W/ EMERGENCY EQUIP".to_string(),
            "3000" => "PUBLIC PEACE VIOLATION SELL/ADVERTISE FIREWORKS".to_string(),
            "3200" => "PUBLIC PEACE VIOLATION ARMED VIOLENCE".to_string(),
            "3300" => "PUBLIC PEACE VIOLATION PUBLIC DEMONSTRATION".to_string(),
            "3400" => "PUBLIC PEACE VIOLATION LOOTING".to_string(),
            "3770" => "INTERFERE WITH PUBLIC OFFICER CONTRABAND IN PRISON".to_string(),
            "3800" => "INTERFERE WITH PUBLIC OFFICER INTERFERENCE JUDICIAL PROCESS".to_string(),
            "3910" => "INTERFERE WITH PUBLIC OFFICER BRIBERY".to_string(),
            "3920" => "INTERFERE WITH PUBLIC OFFICER OFFICIAL MISCONDUCT".to_string(),
            "3960" => "INTIMIDATION INTIMIDATION".to_string(),
            "3966" => "INTIMIDATION EDUCATIONAL INTIMIDAITON".to_string(),
            "3970" => "INTIMIDATION EXTORTION".to_string(),
            "3975" => "INTIMIDATION COMPELLING ORG MEMBERSHIP".to_string(),
            "3980" => "INTIMIDATION COMPELLING CONFESSION".to_string(),
            "4210" => "KIDNAPPING KIDNAPPING".to_string(),
            "4220" => "KIDNAPPING AGGRAVATED".to_string(),
            "4230" => "KIDNAPPING UNLAWFUL RESTRAINT".to_string(),
            "4240" => "KIDNAPPING FORCIBLE DETENTION".to_string(),
            "4255" => "KIDNAPPING UNLAWFUL INTERFERE/VISITATION".to_string(),
            "4310" => "OTHER OFFENSE POSSESSION OF BURGLARY TOOLS".to_string(),
            "4387" => "OTHER OFFENSE VIOLATE ORDER OF PROTECTION".to_string(),
            "4388" => "OTHER OFFENSE VIO BAIL BOND: DOM VIOLENCE".to_string(),
            "4410" => "OTHER OFFENSE DESTRUCTION OF DRAFT CARD".to_string(),
            "4420" => "OTHER OFFENSE CRIMINAL FORTIFICATION".to_string(),
            "4510" => "OTHER OFFENSE PROBATION VIOLATION".to_string(),
            "4625" => "OTHER OFFENSE PAROLE VIOLATION".to_string(),
            "4650" => "OTHER OFFENSE SEX OFFENDER: FAIL TO REGISTER".to_string(),
            "4651" => "OTHER OFFENSE SEX OFFENDER: FAIL REG NEW ADD".to_string(),
            "4652" => "OTHER OFFENSE SEX OFFENDER: PROHIBITED ZONE".to_string(),
            "4740" => "OTHER OFFENSE UNLAWFUL USE OF BODY ARMOR".to_string(),
            "4750" => "OTHER OFFENSE DISCLOSE DV VICTIM LOCATION".to_string(),
            "4800" => "OTHER OFFENSE MONEY LAUNDERING".to_string(),
            "4810" => "OTHER OFFENSE COMPOUNDING A CRIME".to_string(),
            "4860" => "OTHER OFFENSE BOARD PLANE WITH WEAPON".to_string(),
            "5000" => "OTHER OFFENSE OTHER CRIME AGAINST PERSON".to_string(),
            "5001" => "OTHER OFFENSE OTHER CRIME INVOLVING PROPERTY".to_string(),
            "5002" => "OTHER OFFENSE OTHER VEHICLE OFFENSE".to_string(),
            "5003" => "OTHER OFFENSE OTHER ARSON/EXPLOSIVE INCIDENT".to_string(),
            "5007" => "OTHER OFFENSE OTHER WEAPONS VIOLATION".to_string(),
            "5008" => "OTHER OFFENSE FIREARM REGISTRATION VIOLATION".to_string(),
            "500E" => "OTHER OFFENSE EAVESDROPPING".to_string(),
            "500N" => "OTHER OFFENSE ABUSE/NEGLECT: CARE FACILITY".to_string(),
            "5011" => "OTHER OFFENSE LICENSE VIOLATION".to_string(),
            "501A" => "OTHER OFFENSE ANIMAL ABUSE/NEGLECT".to_string(),
            "501H" => "OTHER OFFENSE HAZARDOUS MATERIALS VIOLATION".to_string(),
            "502P" => "OTHER OFFENSE FALSE/STOLEN/ALTERED TRP".to_string(),
            "502R" => "OTHER OFFENSE VEHICLE TITLE/REG OFFENSE".to_string(),
            "502T" => "OTHER OFFENSE TAMPER WITH MOTOR VEHICLE".to_string(),

            _ => "UNKNOWN".to_string(),
        }
    }
}
