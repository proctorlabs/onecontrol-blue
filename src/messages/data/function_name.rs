use crate::devices::DeviceEntityType;

use super::*;

enum_with_metadata! {
    FunctionName:u16; name:&'static str:0 {
        Unknown = 0 {"Unknown"},
        DiagnosticTool = 1 {"Diagnostic Tool"},
        MyrvTablet = 2 {"MyRV Tablet"},
        GasWaterHeater = 3 {"Gas Water Heater"},
        ElectricWaterHeater = 4 {"Electric Water Heater"},
        WaterPump = 5 {"Water Pump"},
        BathVent = 6 {"Bath Vent"},
        Light = 7 {"Light"},
        FloodLight = 8 {"Flood Light"},
        WorkLight = 9 {"Work Light"},
        FrontBedroomCeilingLight = 10 {"Front Bedroom Ceiling Light"},
        FrontBedroomOverheadLight = 11 {"Front Bedroom Overhead Light"},
        FrontBedroomVanityLight = 12 {"Front Bedroom Vanity Light"},
        FrontBedroomSconceLight = 13 {"Front Bedroom Sconce Light"},
        FrontBedroomLoftLight = 14 {"Front Bedroom Loft Light"},
        RearBedroomCeilingLight = 15 {"Rear Bedroom Ceiling Light"},
        RearBedroomOverheadLight = 16 {"Rear Bedroom Overhead Light"},
        RearBedroomVanityLight = 17 {"Rear Bedroom Vanity Light"},
        RearBedroomSconceLight = 18 {"Rear Bedroom Sconce Light"},
        RearBedroomLoftLight = 19 {"Rear Bedroom Loft Light"},
        LoftLight = 20 {"Loft Light"},
        FrontHallLight = 21 {"Front Hall Light"},
        RearHallLight = 22 {"Rear Hall Light"},
        FrontBathroomLight = 23 {"Front Bathroom Light"},
        FrontBathroomVanityLight = 24 {"Front Bathroom Vanity Light"},
        FrontBathroomCeilingLight = 25 {"Front Bathroom Ceiling Light"},
        FrontBathroomShowerLight = 26 {"Front Bathroom Shower Light"},
        FrontBathroomSconceLight = 27 {"Front Bathroom Sconce Light"},
        RearBathroomVanityLight = 28 {"Rear Bathroom Vanity Light"},
        RearBathroomCeilingLight = 29 {"Rear Bathroom Ceiling Light"},
        RearBathroomShowerLight = 30 {"Rear Bathroom Shower Light"},
        RearBathroomSconceLight = 31 {"Rear Bathroom Sconce Light"},
        KitchenCeilingLight = 32 {"Kitchen Ceiling Light"},
        KitchenSconceLight = 33 {"Kitchen Sconce Light"},
        KitchenPendantsLight = 34 {"Kitchen Pendants Light"},
        KitchenRangeLight = 35 {"Kitchen Range Light"},
        KitchenCounterLight = 36 {"Kitchen Counter Light"},
        KitchenBarLight = 37 {"Kitchen Bar Light"},
        KitchenIslandLight = 38 {"Kitchen Island Light"},
        KitchenChandelierLight = 39 {"Kitchen Chandelier Light"},
        KitchenUnderCabinetLight = 40 {"Kitchen Under-Cabinet Light"},
        LivingRoomCeilingLight = 41 {"Living Room Ceiling Light"},
        LivingRoomSconceLight = 42 {"Living Room Sconce Light"},
        LivingRoomPendantsLight = 43 {"Living Room Pendants Light"},
        LivingRoomBarLight = 44 {"Living Room Bar Light"},
        GarageCeilingLight = 45 {"Garage Ceiling Light"},
        GarageCabinetLight = 46 {"Garage Cabinet Light"},
        SecurityLight = 47 {"Security Light"},
        PorchLight = 48 {"Porch Light"},
        AwningLight = 49 {"Awning Light"},
        BathroomLight = 50 {"Bathroom Light"},
        BathroomVanityLight = 51 {"Bathroom Vanity Light"},
        BathroomCeilingLight = 52 {"Bathroom Ceiling Light"},
        BathroomShowerLight = 53 {"Bathroom Shower Light"},
        BathroomSconceLight = 54 {"Bathroom Sconce Light"},
        HallLight = 55 {"Hall Light"},
        BunkRoomLight = 56 {"Bunk Room Light"},
        BedroomLight = 57 {"Bedroom Light"},
        LivingRoomLight = 58 {"Living Room Light"},
        KitchenLight = 59 {"Kitchen Light"},
        LoungeLight = 60 {"Lounge Light"},
        CeilingLight = 61 {"Ceiling Light"},
        EntryLight = 62 {"Entry Light"},
        BedCeilingLight = 63 {"Bed Ceiling Light"},
        BedroomLavLight = 64 {"Bedroom Lavatory Light"},
        ShowerLight = 65 {"Shower Light"},
        GalleyLight = 66 {"Galley Light"},
        FreshTank = 67 {"Fresh Tank"},
        GreyTank = 68 {"Grey Tank"},
        BlackTank = 69 {"Black Tank"},
        FuelTank = 70 {"Fuel Tank"},
        GeneratorFuelTank = 71 {"Generator Fuel Tank"},
        AuxilliaryFuelTank = 72 {"Auxilliary Fuel Tank"},
        FrontBathGreyTank = 73 {"Front Bath Grey Tank"},
        FrontBathFreshTank = 74 {"Front Bath Fresh Tank"},
        FrontBathBlackTank = 75 {"Front Bath Black Tank"},
        RearBathGreyTank = 76 {"Rear Bath Grey Tank"},
        RearBathFreshTank = 77 {"Rear Bath Fresh Tank"},
        RearBathBlackTank = 78 {"Rear Bath Black Tank"},
        MainBathGreyTank = 79 {"Main Bath Grey Tank"},
        MainBathFreshTank = 80 {"Main Bath Fresh Tank"},
        MainBathBlackTank = 81 {"Main Bath Black Tank"},
        GalleyGreyTank = 82 {"Galley Grey Tank"},
        GalleyFreshTank = 83 {"Galley Fresh Tank"},
        GalleyBlackTank = 84 {"Galley Black Tank"},
        KitchenGreyTank = 85 {"Kitchen Grey Tank"},
        KitchenFreshTank = 86 {"Kitchen Fresh Tank"},
        KitchenBlackTank = 87 {"Kitchen Black Tank"},
        LandingGear = 88 {"Landing Gear"},
        FrontStabilizer = 89 {"Front Stabilizer"},
        RearStabilizer = 90 {"Rear Stabilizer"},
        TvLift = 91 {"TV Lift"},
        BedLift = 92 {"Bed Lift"},
        BathVentCover = 93 {"Bath Vent Cover"},
        DoorLock = 94 {"Door Lock"},
        Generator = 95 {"Generator"},
        Slide = 96 {"Slide"},
        MainSlide = 97 {"Main Slide"},
        BedroomSlide = 98 {"Bedroom Slide"},
        GalleySlide = 99 {"Galley Slide"},
        KitchenSlide = 100 {"Kitchen Slide"},
        ClosetSlide = 101 {"Closet Slide"},
        OptionalSlide = 102 {"Optional Slide"},
        DoorSideSlide = 103 {"Door Side Slide"},
        OffDoorSlide = 104 {"Off Door Slide"},
        Awning = 105 {"Awning"},
        LevelUpLeveler = 106 {"Level Up Leveler"},
        WaterTankHeater = 107 {"Water Tank Heater"},
        MyrvTouchscreen = 108 {"MyRV Touchscreen"},
        Leveler = 109 {"Leveler"},
        VentCover = 110 {"Vent Cover"},
        FrontBedroomVentCover = 111 {"Front Bedroom Vent Cover"},
        BedroomVentCover = 112 {"Bedroom Vent Cover"},
        FrontBathroomVentCover = 113 {"Front Bathroom Vent Cover"},
        MainBathroomVentCover = 114 {"Main Bathroom Vent Cover"},
        RearBathroomVentCover = 115 {"Rear Bathroom Vent Cover"},
        KitchenVentCover = 116 {"Kitchen Vent Cover"},
        LivingRoomVentCover = 117 {"Living Room Vent Cover"},
        FourLegTruckCamplerLeveler = 118 {"Four Leg Truck Campler Leveler"},
        SixLegHallEffectEjLeveler = 119 {"Six Leg Hall Effect Ej Leveler"},
        PatioLight = 120 {"Patio Light"},
        HutchLight = 121 {"Hutch Light"},
        ScareLight = 122 {"Scare Light"},
        DinetteLight = 123 {"Dinette Light"},
        BarLight = 124 {"Bar Light"},
        OverheadLight = 125 {"Overhead Light"},
        OverheadBarLight = 126 {"Overhead Bar Light"},
        FoyerLight = 127 {"Foyer Light"},
        RampDoorLight = 128 {"Ramp Door Light"},
        EntertainmentLight = 129 {"Entertainment Light"},
        RearEntryDoorLight = 130 {"Rear Entry Door Light"},
        CeilingFanLight = 131 {"Ceiling Fan Light"},
        OverheadFanLight = 132 {"Overhead Fan Light"},
        BunkSlide = 133 {"Bunk Slide"},
        BedSlide = 134 {"Bed Slide"},
        WardrobeSlide = 135 {"Wardrobe Slide"},
        EntertainmentSlide = 136 {"Entertainment Slide"},
        SofaSlide = 137 {"Sofa Slide"},
        PatioAwning = 138 {"Patio Awning"},
        RearAwning = 139 {"Rear Awning"},
        SideAwning = 140 {"Side Awning"},
        Jacks = 141 {"Jacks"},
        Leveler2 = 142 {"Leveler2"},
        ExteriorLight = 143 {"Exterior Light"},
        LowerAccentLight = 144 {"Lower Accent Light"},
        UpperAccentLight = 145 {"Upper Accent Light"},
        DsSecurityLight = 146 {"Ds Security Light"},
        OdsSecurityLight = 147 {"Ods Security Light"},
        SlideInSlide = 148 {"Slide In Slide"},
        HitchLight = 149 {"Hitch Light"},
        Clock = 150 {"Clock"},
        Tv = 151 {"Tv"},
        Dvd = 152 {"Dvd"},
        BluRay = 153 {"BluRay"},
        Vcr = 154 {"Vcr"},
        Pvr = 155 {"Pvr"},
        Cable = 156 {"Cable"},
        Satellite = 157 {"Satellite"},
        Audio = 158 {"Audio"},
        CdPlayer = 159 {"CdPlayer"},
        Tuner = 160 {"Tuner"},
        Radio = 161 {"Radio"},
        Speakers = 162 {"Speakers"},
        Game = 163 {"Game"},
        ClockRadio = 164 {"Clock Radio"},
        Aux = 165 {"Aux"},
        ClimateZone = 166 {"Climate Zone"},
        Fireplace = 167 {"Fireplace"},
        Thermostat = 168 {"Thermostat"},
        FrontCapLight = 169 {"Front Cap Light"},
        StepLight = 170 {"Step Light"},
        DsFloodLight = 171 {"Ds Flood Light"},
        InteriorLight = 172 {"Interior Light"},
        FreshTankHeater = 173 {"Fresh Tank Heater"},
        GreyTankHeater = 174 {"Grey Tank Heater"},
        BlackTankHeater = 175 {"Black Tank Heater"},
        LpTank = 176 {"LP Tank"},
        StallLight = 177 {"Stall Light"},
        MainLight = 178 {"Main Light"},
        BathLight = 179 {"Bath Light"},
        BunkLight = 180 {"Bunk Light"},
        BedLight = 181 {"Bed Light"},
        CabinetLight = 182 {"Cabinet Light"},
        NetworkBridge = 183 {"Network Bridge"},
        EthernetBridge = 184 {"Ethernet Bridge"},
        WifiBridge = 185 {"Wifi Bridge"},
        InTransitPowerDisconnect = 186 {"In-Transit Power Disconnect"},
        LevelUpUnity = 187 {"Level Up Unity"},
        TtLeveler = 188 {"TT Leveler"},
        TravelTrailerLeveler = 189 {"Travel Trailer Leveler"},
        FifthWheelLeveler = 190 {"Fifth Wheel Leveler"},
        FuelPump = 191 {"Fuel Pump"},
        MainClimateZone = 192 {"Main Climate Zone"},
        BedroomClimateZone = 193 {"Bedroom Climate Zone"},
        GarageClimateZone = 194 {"Garage Climate Zone"},
        CompartmentLight = 195 {"Compartment Light"},
        TrunkLight = 196 {"Trunk Light"},
        BarTv = 197 {"Bar TV"},
        BathroomTv = 198 {"Bathroom TV"},
        BedroomTv = 199 {"Bedroom TV"},
        BunkRoomTv = 200 {"Bunk Room TV"},
        ExteriorTv = 201 {"Exterior TV"},
        FrontBathroomTv = 202 {"Front Bathroom TV"},
        FrontBedroomTv = 203 {"Front Bedroom TV"},
        GarageTv = 204 {"Garage TV"},
        KitchenTv = 205 {"Kitchen TV"},
        LivingRoomTv = 206 {"Living Room TV"},
        LoftTv = 207 {"Loft TV"},
        LoungeTv = 208 {"Lounge TV"},
        MainTv = 209 {"Main TV"},
        PatioTv = 210 {"Patio TV"},
        RearBathroomTv = 211 {"Rear Bathroom TV"},
        RearBedroomTv = 212 {"Rear Bedroom TV"},
        BathroomDoorLock = 213 {"Bathroom Door Lock"},
        BedroomDoorLock = 214 {"Bedroom Door Lock"},
        FrontDoorLock = 215 {"Front Door Lock"},
        GarageDoorLock = 216 {"Garage Door Lock"},
        MainDoorLock = 217 {"Main Door Lock"},
        PatioDoorLock = 218 {"Patio Door Lock"},
        RearDoorLock = 219 {"Rear Door Lock"},
        AccentLight = 220 {"Accent Light"},
        BathroomAccentLight = 221 {"Bathroom Accent Light"},
        BedroomAccentLight = 222 {"Bedroom Accent Light"},
        FrontBedroomAccentLight = 223 {"Front Bedroom Accent Light"},
        GarageAccentLight = 224 {"Garage Accent Light"},
        KitchenAccentLight = 225 {"Kitchen Accent Light"},
        PatioAccentLight = 226 {"Patio Accent Light"},
        RearBedroomAccentLight = 227 {"Rear Bedroom Accent Light"},
        BedroomRadio = 228 {"Bedroom Radio"},
        BunkRoomRadio = 229 {"Bunk Room Radio"},
        ExteriorRadio = 230 {"Exterior Radio"},
        FrontBedroomRadio = 231 {"Front Bedroom Radio"},
        GarageRadio = 232 {"Garage Radio"},
        KitchenRadio = 233 {"Kitchen Radio"},
        LivingRoomRadio = 234 {"Living Room Radio"},
        LoftRadio = 235 {"Loft Radio"},
        PatioRadio = 236 {"Patio Radio"},
        RearBedroomRadio = 237 {"Rear Bedroom Radio"},
        BedroomEntertainmentSystem = 238 {"Bedroom Entertainment System"},
        BunkRoomEntertainmentSystem = 239 {"BunkRoom Entertainment System"},
        EntertainmentSystem = 240 {"Entertainment System"},
        ExteriorEntertainmentSystem = 241 {"Exterior Entertainment System"},
        FrontBedroomEntertainmentSystem = 242 {"Front Bedroom Entertainment System"},
        GarageEntertainmentSystem = 243 {"Garage Entertainment System"},
        KitchenEntertainmentSystem = 244 {"Kitchen Entertainment System"},
        LivingRoomEntertainmentSystem = 245 {"Living Room Entertainment System"},
        LoftEntertainmentSystem = 246 {"Loft Entertainment System"},
        MainEntertainmentSystem = 247 {"Main Entertainment System"},
        PatioEntertainmentSystem = 248 {"Patio Entertainment System"},
        RearBedroomEntertainmentSystem = 249 {"Rear Bedroom Entertainment System"},
        LeftStabilizer = 250 {"Left Stabilizer"},
        RightStabilizer = 251 {"Right Stabilizer"},
        Stabilizer = 252 {"Stabilizer"},
        Solar = 253 {"Solar"},
        SolarPower = 254 {"Solar Power"},
        Battery = 255 {"Battery"},
        MainBattery = 256 {"Main Battery"},
        AuxBattery = 257 {"Aux Battery"},
        ShorePower = 258 {"Shore Power"},
        AcPower = 259 {"AC Power"},
        AcMains = 260 {"AC Mains"},
        AuxPower = 261 {"Aux Power"},
        Outputs = 262 {"Outputs"},
        RampDoor = 263 {"Ramp Door"},
        Fan = 264 {"Fan"},
        BathFan = 265 {"Bath Fan"},
        RearFan = 266 {"Rear Fan"},
        FrontFan = 267 {"Front Fan"},
        KitchenFan = 268 {"Kitchen Fan"},
        CeilingFan = 269 {"Ceiling Fan"},
        TankHeater = 270 {"Tank Heater"},
        FrontCeilingLight = 271 {"Front Ceiling Light"},
        RearCeilingLight = 272 {"Rear Ceiling Light"},
        CargoLight = 273 {"Cargo Light"},
        FasciaLight = 274 {"Fascia Light"},
        SlideCeilingLight = 275 {"Slide Ceiling Light"},
        SlideOverheadLight = 276 {"Slide Overhead Light"},
        DecorLight = 277 {"Decor Light"},
        ReadingLight = 278 {"Reading Light"},
        FrontReadingLight = 279 {"Front Reading Light"},
        RearReadingLight = 280 {"Rear Reading Light"},
        LivingRoomClimateZone = 281 {"Living Room Climate Zone"},
        FrontLivingRoomClimateZone = 282 {"Front LivingRoom Climate Zone"},
        RearLivingRoomClimateZone = 283 {"Rear LivingRoom Climate Zone"},
        FrontBedroomClimateZone = 284 {"Front Bedroom Climate Zone"},
        RearBedroomClimateZone = 285 {"Rear Bedroom Climate Zone"},
        BedTilt = 286 {"Bed Tilt"},
        FrontBedTilt = 287 {"Front Bed Tilt"},
        RearBedTilt = 288 {"Rear Bed Tilt"},
        MensLight = 289 {"Mens Light"},
        WomensLight = 290 {"Womens Light"},
        ServiceLight = 291 {"Service Light"},
        OdsFloodLight = 292 {"ODS Flood Light"},
        UnderbodyAccentLight = 293 {"Underbody Accent Light"},
        SpeakerLight = 294 {"Speaker Light"},
        WaterHeater = 295 {"Water Heater"},
        WaterHeaters = 296 {"Water Heaters"},
        Aquafi = 297 {"Aquafi"},
        ConnectAnywhere = 298 {"Connect Anywhere"},
        SlideIfEquip = 299 {"Slide If Equip"},
        AwningIfEquip = 300 {"Awning If Equip"},
        AwningLightIfEquip = 301 {"Awning Light If Equip"},
        InteriorLightIfEquip = 302 {"Interior Light If Equip"},
        WasteValve = 303 {"Waste Valve"},
        TireLinc = 304 {"Tire Linc"},
        FrontLockerLight = 305 {"Front Locker Light"},
        RearLockerLight = 306 {"Rear Locker Light"},
        RearAuxPower = 307 {"Rear Aux Power"},
        RockLight = 308 {"Rock Light"},
        ChassisLight = 309 {"Chassis Light"},
        ExteriorShowerLight = 310 {"Exterior Shower Light"},
        LivingRoomAccentLight = 311 {"Living Room Accent Light"},
        RearFloodLight = 312 {"Rear Flood Light"},
        PassengerFloodLight = 313 {"Passenger Flood Light"},
        DriverFloodLight = 314 {"Driver Flood Light"},
        BathroomSlide = 315 {"Bathroom Slide"},
        RoofLift = 316 {"Roof Lift"},
        YetiPackage = 317 {"Yeti Package"},
        PropaneLocker = 318 {"Propane Locker"},
        GarageAwning = 319 {"Garage Awning"},
        MonitorPanel = 320 {"Monitor Panel"},
        Camera = 321 {"Camera"},
        JaycoAusTbbGw = 322 {"Jayco Aus TBB Gw"},
        GatewayRvlink = 323 {"Gateway RVLink"},
        AccessoryTemperature = 324 {"Accessory Temperature"},
        AccessoryRefrigerator = 325 {"Accessory Refrigerator"},
        AccessoryFridge = 326 {"Accessory Fridge"},
        AccessoryFreezer = 327 {"Accessory Freezer"},
        AccessoryExternal = 328 {"Accessory External"},
        TrailerBrakeController = 329 {"Trailer Brake Controller"},
        TempRefrigerator = 330 {"Temp Refrigerator"},
        TempRefrigeratorHome = 331 {"Temp Refrigerator Home"},
        TempFreezer = 332 {"Temp Freezer"},
        TempFreezerHome = 333 {"Temp Freezer Home"},
        TempCooler = 334 {"Temp Cooler"},
        TempKitchen = 335 {"Temp Kitchen"},
        TempLivingRoom = 336 {"Temp Living Room"},
        TempBedroom = 337 {"Temp Bedroom"},
        TempMasterBedroom = 338 {"Temp Master Bedroom"},
        TempGarage = 339 {"Temp Garage"},
        TempBasement = 340 {"Temp Basement"},
        TempBathroom = 341 {"Temp Bathroom"},
        TempStorageArea = 342 {"Temp Storage Area"},
        TempDriversArea = 343 {"Temp Drivers Area"},
        TempBunks = 344 {"Temp Bunks"},
        LpTankRv = 345 {"LP Tank Rv"},
        LpTankHome = 346 {"LP Tank Home"},
        LpTankCabin = 347 {"LP Tank Cabin"},
        LpTankBbq = 348 {"LP Tank BBQ"},
        LpTankGrill = 349 {"Lp Tank Grill"},
        LpTankSubmarine = 350 {"LP Tank Submarine"},
        LpTankOther = 351 {"LP Tank Other"},
        AntiLockBrakingSystem = 352 {"Anti-Lock Braking System"},
        LocapGateway = 353 {"Locap Gateway"},
        Bootloader = 354 {"Bootloader"},
        AuxiliaryBattery = 355 {"Auxiliary Battery"},
        ChassisBattery = 356 {"Chassis Battery"},
        HouseBattery = 357 {"House Battery"},
        KitchenBattery = 358 {"Kitchen Battery"},
        ElectronicSwayControl = 359 {"Electronic Sway Control"},
        JacksLights = 360 {"Jacks Lights"},
        AwningSensor = 361 {"Awning Sensor"},
        InteriorStepLight = 362 {"Interior Step Light"},
        ExteriorStepLight = 363 {"Exterior Step Light"},
        WifiBooster = 364 {"Wifi Booster"},
    }
}

impl std::fmt::Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FunctionName {
    pub fn device_entity_type(&self) -> DeviceEntityType {
        match self {
            FunctionName::Slide
            | FunctionName::MainSlide
            | FunctionName::BedroomSlide
            | FunctionName::GalleySlide
            | FunctionName::KitchenSlide
            | FunctionName::BunkSlide
            | FunctionName::BedSlide
            | FunctionName::WardrobeSlide
            | FunctionName::EntertainmentSlide
            | FunctionName::SofaSlide
            | FunctionName::ClosetSlide
            | FunctionName::SlideInSlide
            | FunctionName::HitchLight
            | FunctionName::OptionalSlide
            | FunctionName::DoorSideSlide
            | FunctionName::OffDoorSlide => DeviceEntityType::Slide,
            FunctionName::FuelTank
            | FunctionName::GeneratorFuelTank
            | FunctionName::AuxilliaryFuelTank => DeviceEntityType::FuelTank,
            FunctionName::FreshTank
            | FunctionName::BlackTank
            | FunctionName::RearBathBlackTank
            | FunctionName::MainBathBlackTank
            | FunctionName::FrontBathBlackTank
            | FunctionName::GalleyBlackTank
            | FunctionName::KitchenBlackTank => DeviceEntityType::BlackTank,
            FunctionName::FrontBathGreyTank
            | FunctionName::RearBathGreyTank
            | FunctionName::MainBathGreyTank
            | FunctionName::GalleyGreyTank
            | FunctionName::GreyTank
            | FunctionName::KitchenGreyTank => DeviceEntityType::GreyTank,
            FunctionName::RearBathFreshTank
            | FunctionName::FrontBathFreshTank
            | FunctionName::GalleyFreshTank
            | FunctionName::MainBathFreshTank
            | FunctionName::KitchenFreshTank => DeviceEntityType::FreshTank,
            FunctionName::Light
            | FunctionName::FloodLight
            | FunctionName::WorkLight
            | FunctionName::FrontBedroomCeilingLight
            | FunctionName::FrontBedroomOverheadLight
            | FunctionName::FrontBedroomVanityLight
            | FunctionName::FrontBedroomSconceLight
            | FunctionName::FrontBedroomLoftLight
            | FunctionName::LoftLight
            | FunctionName::FrontHallLight
            | FunctionName::RearHallLight
            | FunctionName::FrontBathroomLight
            | FunctionName::FrontBathroomVanityLight
            | FunctionName::FrontBathroomCeilingLight
            | FunctionName::FrontBathroomShowerLight
            | FunctionName::FrontBathroomSconceLight
            | FunctionName::RearBathroomVanityLight
            | FunctionName::RearBathroomCeilingLight
            | FunctionName::RearBathroomShowerLight
            | FunctionName::RearBathroomSconceLight
            | FunctionName::KitchenCeilingLight
            | FunctionName::KitchenSconceLight
            | FunctionName::KitchenPendantsLight
            | FunctionName::KitchenRangeLight
            | FunctionName::KitchenCounterLight
            | FunctionName::KitchenBarLight
            | FunctionName::KitchenIslandLight
            | FunctionName::KitchenChandelierLight
            | FunctionName::KitchenUnderCabinetLight
            | FunctionName::LivingRoomCeilingLight
            | FunctionName::LivingRoomSconceLight
            | FunctionName::LivingRoomPendantsLight
            | FunctionName::LivingRoomBarLight
            | FunctionName::GarageCeilingLight
            | FunctionName::GarageCabinetLight
            | FunctionName::SecurityLight
            | FunctionName::PorchLight
            | FunctionName::AwningLight
            | FunctionName::StallLight
            | FunctionName::MainLight
            | FunctionName::BathLight
            | FunctionName::BunkLight
            | FunctionName::BedLight
            | FunctionName::CabinetLight
            | FunctionName::BathroomLight
            | FunctionName::BathroomVanityLight
            | FunctionName::BathroomCeilingLight
            | FunctionName::BathroomShowerLight
            | FunctionName::BathroomSconceLight
            | FunctionName::HallLight
            | FunctionName::BunkRoomLight
            | FunctionName::BedroomLight
            | FunctionName::LivingRoomLight
            | FunctionName::KitchenLight
            | FunctionName::LoungeLight
            | FunctionName::CeilingLight
            | FunctionName::EntryLight
            | FunctionName::BedCeilingLight
            | FunctionName::BedroomLavLight
            | FunctionName::ShowerLight
            | FunctionName::FrontCeilingLight
            | FunctionName::RearCeilingLight
            | FunctionName::CargoLight
            | FunctionName::FasciaLight
            | FunctionName::SlideCeilingLight
            | FunctionName::SlideOverheadLight
            | FunctionName::DecorLight
            | FunctionName::ReadingLight
            | FunctionName::FrontReadingLight
            | FunctionName::RearReadingLight
            | FunctionName::GalleyLight
            | FunctionName::RearBedroomCeilingLight
            | FunctionName::RockLight
            | FunctionName::ChassisLight
            | FunctionName::ExteriorShowerLight
            | FunctionName::LivingRoomAccentLight
            | FunctionName::RearFloodLight
            | FunctionName::PassengerFloodLight
            | FunctionName::DriverFloodLight
            | FunctionName::RearBedroomOverheadLight
            | FunctionName::RearBedroomVanityLight
            | FunctionName::AccentLight
            | FunctionName::BathroomAccentLight
            | FunctionName::BedroomAccentLight
            | FunctionName::FrontBedroomAccentLight
            | FunctionName::GarageAccentLight
            | FunctionName::KitchenAccentLight
            | FunctionName::PatioAccentLight
            | FunctionName::RearBedroomAccentLight
            | FunctionName::RearBedroomSconceLight
            | FunctionName::PatioLight
            | FunctionName::HutchLight
            | FunctionName::ScareLight
            | FunctionName::DinetteLight
            | FunctionName::BarLight
            | FunctionName::OverheadLight
            | FunctionName::OverheadBarLight
            | FunctionName::FoyerLight
            | FunctionName::FrontCapLight
            | FunctionName::StepLight
            | FunctionName::DsFloodLight
            | FunctionName::InteriorLight
            | FunctionName::RampDoorLight
            | FunctionName::EntertainmentLight
            | FunctionName::RearEntryDoorLight
            | FunctionName::CeilingFanLight
            | FunctionName::ExteriorLight
            | FunctionName::LowerAccentLight
            | FunctionName::UpperAccentLight
            | FunctionName::DsSecurityLight
            | FunctionName::OdsSecurityLight
            | FunctionName::OverheadFanLight
            | FunctionName::RearBedroomLoftLight => DeviceEntityType::LightSwitch,
            FunctionName::Unknown
            | FunctionName::DiagnosticTool
            | FunctionName::MyrvTablet
            | FunctionName::GasWaterHeater
            | FunctionName::ElectricWaterHeater
            | FunctionName::WaterPump
            | FunctionName::BathVent
            | FunctionName::LandingGear
            | FunctionName::FrontStabilizer
            | FunctionName::RearStabilizer
            | FunctionName::TvLift
            | FunctionName::BedLift
            | FunctionName::BathVentCover
            | FunctionName::DoorLock
            | FunctionName::Generator
            | FunctionName::Awning
            | FunctionName::LevelUpLeveler
            | FunctionName::WaterTankHeater
            | FunctionName::MyrvTouchscreen
            | FunctionName::Leveler
            | FunctionName::VentCover
            | FunctionName::FrontBedroomVentCover
            | FunctionName::BedroomVentCover
            | FunctionName::FrontBathroomVentCover
            | FunctionName::MainBathroomVentCover
            | FunctionName::RearBathroomVentCover
            | FunctionName::KitchenVentCover
            | FunctionName::LivingRoomVentCover
            | FunctionName::FourLegTruckCamplerLeveler
            | FunctionName::SixLegHallEffectEjLeveler
            | FunctionName::PatioAwning
            | FunctionName::RearAwning
            | FunctionName::SideAwning
            | FunctionName::Jacks
            | FunctionName::Leveler2
            | FunctionName::Clock
            | FunctionName::Tv
            | FunctionName::Dvd
            | FunctionName::BluRay
            | FunctionName::Vcr
            | FunctionName::Pvr
            | FunctionName::Cable
            | FunctionName::Satellite
            | FunctionName::Audio
            | FunctionName::CdPlayer
            | FunctionName::Tuner
            | FunctionName::Radio
            | FunctionName::Speakers
            | FunctionName::Game
            | FunctionName::ClockRadio
            | FunctionName::Aux
            | FunctionName::ClimateZone
            | FunctionName::Fireplace
            | FunctionName::Thermostat
            | FunctionName::FreshTankHeater
            | FunctionName::GreyTankHeater
            | FunctionName::BlackTankHeater
            | FunctionName::LpTank
            | FunctionName::NetworkBridge
            | FunctionName::EthernetBridge
            | FunctionName::WifiBridge
            | FunctionName::InTransitPowerDisconnect
            | FunctionName::LevelUpUnity
            | FunctionName::TtLeveler
            | FunctionName::TravelTrailerLeveler
            | FunctionName::FifthWheelLeveler
            | FunctionName::FuelPump
            | FunctionName::MainClimateZone
            | FunctionName::BedroomClimateZone
            | FunctionName::GarageClimateZone
            | FunctionName::CompartmentLight
            | FunctionName::TrunkLight
            | FunctionName::BarTv
            | FunctionName::BathroomTv
            | FunctionName::BedroomTv
            | FunctionName::BunkRoomTv
            | FunctionName::ExteriorTv
            | FunctionName::FrontBathroomTv
            | FunctionName::FrontBedroomTv
            | FunctionName::GarageTv
            | FunctionName::KitchenTv
            | FunctionName::LivingRoomTv
            | FunctionName::LoftTv
            | FunctionName::LoungeTv
            | FunctionName::MainTv
            | FunctionName::PatioTv
            | FunctionName::RearBathroomTv
            | FunctionName::RearBedroomTv
            | FunctionName::BathroomDoorLock
            | FunctionName::BedroomDoorLock
            | FunctionName::FrontDoorLock
            | FunctionName::GarageDoorLock
            | FunctionName::MainDoorLock
            | FunctionName::PatioDoorLock
            | FunctionName::RearDoorLock
            | FunctionName::BedroomRadio
            | FunctionName::BunkRoomRadio
            | FunctionName::ExteriorRadio
            | FunctionName::FrontBedroomRadio
            | FunctionName::GarageRadio
            | FunctionName::KitchenRadio
            | FunctionName::LivingRoomRadio
            | FunctionName::LoftRadio
            | FunctionName::PatioRadio
            | FunctionName::RearBedroomRadio
            | FunctionName::BedroomEntertainmentSystem
            | FunctionName::BunkRoomEntertainmentSystem
            | FunctionName::EntertainmentSystem
            | FunctionName::ExteriorEntertainmentSystem
            | FunctionName::FrontBedroomEntertainmentSystem
            | FunctionName::GarageEntertainmentSystem
            | FunctionName::KitchenEntertainmentSystem
            | FunctionName::LivingRoomEntertainmentSystem
            | FunctionName::LoftEntertainmentSystem
            | FunctionName::MainEntertainmentSystem
            | FunctionName::PatioEntertainmentSystem
            | FunctionName::RearBedroomEntertainmentSystem
            | FunctionName::LeftStabilizer
            | FunctionName::RightStabilizer
            | FunctionName::Stabilizer
            | FunctionName::Solar
            | FunctionName::SolarPower
            | FunctionName::Battery
            | FunctionName::MainBattery
            | FunctionName::AuxBattery
            | FunctionName::ShorePower
            | FunctionName::AcPower
            | FunctionName::AcMains
            | FunctionName::AuxPower
            | FunctionName::Outputs
            | FunctionName::RampDoor
            | FunctionName::Fan
            | FunctionName::BathFan
            | FunctionName::RearFan
            | FunctionName::FrontFan
            | FunctionName::KitchenFan
            | FunctionName::CeilingFan
            | FunctionName::TankHeater
            | FunctionName::LivingRoomClimateZone
            | FunctionName::FrontLivingRoomClimateZone
            | FunctionName::RearLivingRoomClimateZone
            | FunctionName::FrontBedroomClimateZone
            | FunctionName::RearBedroomClimateZone
            | FunctionName::BedTilt
            | FunctionName::FrontBedTilt
            | FunctionName::RearBedTilt
            | FunctionName::MensLight
            | FunctionName::WomensLight
            | FunctionName::ServiceLight
            | FunctionName::OdsFloodLight
            | FunctionName::UnderbodyAccentLight
            | FunctionName::SpeakerLight
            | FunctionName::WaterHeater
            | FunctionName::WaterHeaters
            | FunctionName::Aquafi
            | FunctionName::ConnectAnywhere
            | FunctionName::SlideIfEquip
            | FunctionName::AwningIfEquip
            | FunctionName::AwningLightIfEquip
            | FunctionName::InteriorLightIfEquip
            | FunctionName::WasteValve
            | FunctionName::TireLinc
            | FunctionName::FrontLockerLight
            | FunctionName::RearLockerLight
            | FunctionName::RearAuxPower
            | FunctionName::BathroomSlide
            | FunctionName::RoofLift
            | FunctionName::YetiPackage
            | FunctionName::PropaneLocker
            | FunctionName::GarageAwning
            | FunctionName::MonitorPanel
            | FunctionName::Camera
            | FunctionName::JaycoAusTbbGw
            | FunctionName::GatewayRvlink
            | FunctionName::AccessoryTemperature
            | FunctionName::AccessoryRefrigerator
            | FunctionName::AccessoryFridge
            | FunctionName::AccessoryFreezer
            | FunctionName::AccessoryExternal
            | FunctionName::TrailerBrakeController
            | FunctionName::TempRefrigerator
            | FunctionName::TempRefrigeratorHome
            | FunctionName::TempFreezer
            | FunctionName::TempFreezerHome
            | FunctionName::TempCooler
            | FunctionName::TempKitchen
            | FunctionName::TempLivingRoom
            | FunctionName::TempBedroom
            | FunctionName::TempMasterBedroom
            | FunctionName::TempGarage
            | FunctionName::TempBasement
            | FunctionName::TempBathroom
            | FunctionName::TempStorageArea
            | FunctionName::TempDriversArea
            | FunctionName::TempBunks
            | FunctionName::LpTankRv
            | FunctionName::LpTankHome
            | FunctionName::LpTankCabin
            | FunctionName::LpTankBbq
            | FunctionName::LpTankGrill
            | FunctionName::LpTankSubmarine
            | FunctionName::LpTankOther
            | FunctionName::AntiLockBrakingSystem
            | FunctionName::LocapGateway
            | FunctionName::Bootloader
            | FunctionName::AuxiliaryBattery
            | FunctionName::ChassisBattery
            | FunctionName::HouseBattery
            | FunctionName::KitchenBattery
            | FunctionName::ElectronicSwayControl
            | FunctionName::JacksLights
            | FunctionName::InteriorStepLight
            | FunctionName::ExteriorStepLight
            | FunctionName::AwningSensor
            | FunctionName::WifiBooster => DeviceEntityType::None,
        }
    }
}
