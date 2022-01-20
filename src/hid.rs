pub mod descriptor {
    use std::{collections::HashMap, fmt};

    #[derive(Debug)]
    pub struct Report {
        pub items: Vec<HidItem>,
    }
    
    impl Report {
        pub fn new() -> Self {
            let report = Report {
                items: vec![],
            };

            report
        }
    }

    #[derive(Debug)]
    pub struct HidItem {
        pub item_type: HidItemType,
        pub size_bytes: u8,
        pub usage_page: Option<u32>,
        pub main_tag: Option<HidMainTag>,
        pub global_tag: Option<HidGlobalTag>,
        pub local_tag: Option<HidLocalTag>,

        pub bytes: Vec<u8>,
    }

    impl fmt::Display for HidItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let print_data: (String, String) = match &self.item_type {
                HidItemType::Main => ("Main".to_string(), get_main_tag_name(self)),
                HidItemType::Global => (
                    "Global".to_string(),
                    get_global_tag_name(self),
                ),
                HidItemType::Local => (
                    "Local".to_string(),
                    get_local_tag_name(self),
                ),
                HidItemType::Unknown => ("Unknown".to_string(), "".to_string()),
            };

            write!(f, "{} [{}]", print_data.1, print_data.0)
        }
    }

    fn get_main_tag_name(item: &HidItem) -> String {
        return match &item.main_tag {
            Some(tag) => match tag {
                HidMainTag::Input(data) => format!("Input ({})", data),
                HidMainTag::Output(data) => format!("Output ({})", data),
                HidMainTag::Feature(data) => format!("Feature ({})", data),
                HidMainTag::Collection(col_type) => format!("Collection ({})", col_type),
                HidMainTag::EndCollection => format!("End Collection"),
                HidMainTag::Reserved => "Reserved".to_string(),
            },
            None => "".to_string(),
        };
    }

    fn get_global_tag_name(item: &HidItem) -> String {
        return match &item.global_tag {
            Some(tag) => match tag {
                // TODO: Should UsagePage be u16?
                HidGlobalTag::UsagePage(value) => format!("Usage Page ({})", super::name::get_usage_page_name(*value)),
                HidGlobalTag::LogicalMinimum(value) => format!("Logical Minimum ({})", value),
                HidGlobalTag::LogicalMaximum(value) => format!("Logical Maximum ({})", value),
                HidGlobalTag::PhysicalMinimum(value) => format!("Physical Minimum ({})", value),
                HidGlobalTag::PhysicalMaximum(value) => format!("Physical Maximum ({})", value),
                HidGlobalTag::UnitExponent(value) => format!("Unit Exponent ({})", value),
                HidGlobalTag::Unit(value) => format!("Unit ({})", value),
                HidGlobalTag::ReportSize(value) => format!("Report Size ({})", value),
                HidGlobalTag::ReportId(value) => format!("Report ID ({})", value),
                HidGlobalTag::ReportCount(value) => format!("Report Count ({})", value),
                HidGlobalTag::Push(value) => format!("Push ({})", value),
                HidGlobalTag::Pop(value) => format!("Pop ({})", value),
                HidGlobalTag::Reserved(value) => format!("Reserved ({})", value),
            },
            None => "".to_string(),
        };
    }

    fn get_local_tag_name(item: &HidItem) -> String {
        return match &item.local_tag {
            Some(tag) => match tag {
                HidLocalTag::Usage(value) => format!("Usage ({})", super::name::get_usage_name(item.usage_page, *value)),
                HidLocalTag::UsageMinimum(value) => format!("Usage Minimum ({})", value),
                HidLocalTag::UsageMaximum(value) => format!("Usage Maximum ({})", value),
                HidLocalTag::DesignatorIndex(value) => format!("Designator Index ({})", value),
                HidLocalTag::DesignatorMinimum(value) => format!("Designator Minimum ({})", value),
                HidLocalTag::DesignatorMaximum(value) => format!("Designator Maximum ({})", value),
                HidLocalTag::StringIndex(value) => format!("String Index ({})", value),
                HidLocalTag::StringMinimum(value) => format!("String Minimum ({})", value),
                HidLocalTag::StringMaximum(value) => format!("String Maximum ({})", value),
                HidLocalTag::Delimiter(value) => format!("Delimiter ({})", value),
                HidLocalTag::Reserved(value) => format!("Reserved ({})", value),
            },
            None => "".to_string(),
        };
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum HidItemType {
        Main,
        Global,
        Local,
        Unknown,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum HidMainTag {
        Input(MainInputData),
        Output(MainOutputData),
        Feature(MainOutputData),
        Collection(CollectionType),
        EndCollection,
        Reserved,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum HidGlobalTag {
        UsagePage(u32),
        LogicalMinimum(i32),
        LogicalMaximum(i32),
        PhysicalMinimum(i32),
        PhysicalMaximum(i32),
        UnitExponent(i32),
        Unit(i32),
        ReportSize(u32),
        ReportId(u32),
        ReportCount(u32),
        Push(i32),
        Pop(i32),
        Reserved(i32),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum HidLocalTag {
        Usage(u32),
        UsageMinimum(u32),
        UsageMaximum(u32),
        DesignatorIndex(u32),
        DesignatorMinimum(u32),
        DesignatorMaximum(u32),
        StringIndex(u32),
        StringMinimum(u32),
        StringMaximum(u32),
        Delimiter(u32),
        Reserved(u32),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct MainInputData {
        pub item_type: ItemType,
        pub data_type: DataType,
        pub data_point: DataPoint,
        pub wrapping: Wrapping,
        pub linearity: Linearity,
        pub state_preferrence: StatePreference,
        pub null_state: NullState,
        pub field_type: FieldType,
    }
    
    impl fmt::Display for MainInputData {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let item_type = match self.item_type {
                ItemType::Data => "Data",
                ItemType::Constant => "Constant",
            };
            let data_type = match self.data_type {
                DataType::Array => "Array",
                DataType::Variable => "Variable",
            };
            let data_point = match self.data_point {
                DataPoint::Absolute => "Absolute",
                DataPoint::Relative => "Relative",
            };
            let wrapping = match self.wrapping {
                Wrapping::NoWrap => "No Wrap",
                Wrapping::Wrap => "Wrap",
            };
            let linearity = match self.linearity {
                Linearity::Linear => "Linear",
                Linearity::Nonlinear => "Non-liniear",
            };
            let state_preferrence = match self.state_preferrence {
                StatePreference::PreferredState => "Preferred State",
                StatePreference::NoPreferredState => "No Preferred State",
            };
            let null_state = match self.null_state {
                NullState::NoNullState => "No Null State",
                NullState::NullState => "Null State",
            };
            let field_type = match self.field_type {
                FieldType::BitField => "Bit Field",
                FieldType::BufferedBytes => "BufferedBytes",
            };

            write!(f, "{}, {}, {}, {}, {}, {}, {}, {}", item_type, data_type, data_point, wrapping, linearity, state_preferrence, null_state, field_type)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct MainOutputData {
        pub item_type: ItemType,
        pub data_type: DataType,
        pub data_point: DataPoint,
        pub wrapping: Wrapping,
        pub linearity: Linearity,
        pub state_preferrence: StatePreference,
        pub null_state: NullState,
        pub volatility: Volatility,
        pub field_type: FieldType,
    }

    impl fmt::Display for MainOutputData {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let item_type = match self.item_type {
                ItemType::Data => "Data",
                ItemType::Constant => "Constant",
            };
            let data_type = match self.data_type {
                DataType::Array => "Array",
                DataType::Variable => "Variable",
            };
            let data_point = match self.data_point {
                DataPoint::Absolute => "Absolute",
                DataPoint::Relative => "Relative",
            };
            let wrapping = match self.wrapping {
                Wrapping::NoWrap => "No Wrap",
                Wrapping::Wrap => "Wrap",
            };
            let linearity = match self.linearity {
                Linearity::Linear => "Linear",
                Linearity::Nonlinear => "Non-liniear",
            };
            let state_preferrence = match self.state_preferrence {
                StatePreference::PreferredState => "Preferred State",
                StatePreference::NoPreferredState => "No Preferred State",
            };
            let null_state = match self.null_state {
                NullState::NoNullState => "No Null State",
                NullState::NullState => "Null State",
            };
            let volatility = match self.volatility {
                Volatility::NonVolatile => "Non-volatile",
                Volatility::Volatile => "Volatile",
            };
            let field_type = match self.field_type {
                FieldType::BitField => "Bit Field",
                FieldType::BufferedBytes => "BufferedBytes",
            };

            write!(f, "{}, {}, {}, {}, {}, {}, {}, {}, {}", item_type, data_type, data_point, wrapping, linearity, state_preferrence, null_state, volatility, field_type)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum ItemType {
        Data,
        Constant,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum DataType {
        Array,
        Variable,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum DataPoint {
        Absolute,
        Relative,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Wrapping {
        NoWrap,
        Wrap,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Linearity {
        Linear,
        Nonlinear,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum StatePreference {
        PreferredState,
        NoPreferredState,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum NullState {
        NoNullState,
        NullState,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Volatility {
        NonVolatile,
        Volatile,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum FieldType {
        BitField,
        BufferedBytes,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum CollectionType {
        Physical,
        Application,
        Logical,
        Report,
        NamedArray,
        UsageSwitch,
        UsageModifier,
        Reserved,
    }

    impl fmt::Display for CollectionType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let col_type = match &self {
                CollectionType::Physical => "Physical",
                CollectionType::Application => "Application",
                CollectionType::Logical => "Logical",
                CollectionType::Report => "Report",
                CollectionType::NamedArray => "Named Array",
                CollectionType::UsageSwitch => "Usage Switch",
                CollectionType::UsageModifier => "Usage Modifier",
                CollectionType::Reserved => "Reserved",
            };

            write!(f, "{}", col_type)
        }
    }

    #[derive(Debug)]
    struct HidItemBuilder {
        pub usage_page: Option<u32>,
        pub prefix_bits: u8,
        pub data_bits: Vec<u8>,
    }

    impl HidItemBuilder {
        fn new(usage_page: Option<u32>, prefix_bits: u8) -> Self {
            let builder = HidItemBuilder {
                usage_page: usage_page,
                prefix_bits: prefix_bits,
                data_bits: vec![],
            };

            builder
        }

        fn build(&self) -> HidItem {
            let item_type = self.get_item_type(self.prefix_bits);
            let size_bytes = self.get_size_bytes(self.prefix_bits);

            let tags: (
                Option<HidMainTag>,
                Option<HidGlobalTag>,
                Option<HidLocalTag>,
            ) = match &item_type {
                HidItemType::Main => (Some(self.get_main_tag(self)), None, None),
                HidItemType::Global => (None, Some(self.get_global_tag(self)), None),
                HidItemType::Local => (None, None, Some(self.get_local_tag(self))),
                _ => (None, None, None),
            };

            let mut bytes: Vec<u8> = vec![];
            bytes.push(self.prefix_bits);
            bytes.extend(self.data_bits.iter());
            HidItem {
                item_type: item_type,
                size_bytes: size_bytes,
                usage_page: self.usage_page,
                main_tag: tags.0,
                global_tag: tags.1,
                local_tag: tags.2,
                bytes: bytes,
            }
        }

        fn push_data(&mut self, data: u8) {
            self.data_bits.push(data);
        }

        fn get_size(&self) -> u8 {
            self.get_size_bytes(self.prefix_bits)
        }

        fn get_item_type(&self, prefix_bits: u8) -> HidItemType {
            // Extract bits 2 - 3, e.g. 0b01010101
            //                                ^^
            let item_type_id = (prefix_bits & 0b00001100) >> 2;
            return match item_type_id {
                0 => HidItemType::Main,
                1 => HidItemType::Global,
                2 => HidItemType::Local,
                _ => HidItemType::Unknown,
            };
        }
    
        fn get_size_bytes(&self, prefix_bits: u8) -> u8 {
            // Extract bits 0 - 1, e.g. 0b01010101
            //                                  ^^
            let size_id = prefix_bits & 0b00000011;
            return match size_id {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 4,
                _ => todo!("How to handle illegal values?"),
            };
        }

        fn get_main_tag(&self, item_builder: &HidItemBuilder) -> HidMainTag {
            // Extract bits 4 - 7, e.g. 0b01010101
            //                            ^^^^
            let tag = (item_builder.prefix_bits & 0b11110000) >> 4;
            return match tag {
                0b1000 => HidMainTag::Input(self.get_main_input_data(&item_builder.data_bits)),
                0b1001 => HidMainTag::Output(self.get_main_output_data(&item_builder.data_bits)),
                0b1011 => HidMainTag::Feature(self.get_main_output_data(&item_builder.data_bits)),
                0b1010 => HidMainTag::Collection(self.get_main_collection_type(&item_builder.data_bits)),
                0b1100 => HidMainTag::EndCollection,
                0b1101..=0b1111 => HidMainTag::Reserved,
                _ => todo!("How to handle illegal values?"),
            };
        }
    
        fn get_main_input_data(&self, data_bits: &Vec<u8>) -> MainInputData {
            let first_byte = data_bits[0];
            let second_byte = if data_bits.len() > 1 { data_bits[1] } else { 0b00000000 };

            // Bit 0 {Data (0) | Constant (1)}
            // Bit 1 {Array (0) | Variable (1)}
            // Bit 2 {Absolute (0) | Relative (1)}
            // Bit 3 {No Wrap (0) | Wrap (1)}
            // Bit 4 {Linear (0) | Non Linear (1)}
            // Bit 5 {Preferred State (0) | No Preferred (1)}
            // Bit 6 {No Null position (0) | Null state(1)}
            // Bit 7 Reserved (0)
            // Bit 8 {Bit Field (0) | Buffered Bytes (1)}
            // Bit 31-9 Reserved (0)
            MainInputData {
                item_type: match first_byte & 0b00000001 {
                    0 => ItemType::Data,
                    _ => ItemType::Constant,
                },
                data_type: match (first_byte & 0b00000010) >> 1 {
                    0 => DataType::Array,
                    _ => DataType::Variable,
                },
                data_point: match (first_byte & 0b00000100) >> 2 {
                    0 => DataPoint::Absolute,
                    _ => DataPoint::Relative,
                },
                wrapping: match (first_byte & 0b00001000) >> 3 {
                    0 => Wrapping::NoWrap,
                    _ => Wrapping::Wrap,
                },
                linearity: match (first_byte & 0b00010000) >> 4 {
                    0 => Linearity::Linear,
                    _ => Linearity::Nonlinear,
                },
                state_preferrence: match (first_byte & 0b00100000) >> 5 {
                    0 => StatePreference::PreferredState,
                    _ => StatePreference::NoPreferredState,
                },
                null_state: match (first_byte & 0b01000000) >> 6 {
                    0 => NullState::NoNullState,
                    _ => NullState::NullState,
                },
                field_type: match second_byte & 0b00000001 {
                    0 => FieldType::BitField,
                    _ => FieldType::BufferedBytes,
                },
            }
        }
    
        fn get_main_output_data(&self, data_bits: &Vec<u8>) -> MainOutputData {
            let first_byte = data_bits[0];
            let second_byte = if data_bits.len() > 1 { data_bits[1] } else { 0b00000000 };

            // Bit 0 {Data (0) | Constant (1)}
            // Bit 1 {Array (0) | Variable (1)}
            // Bit 2 {Absolute (0) | Relative (1)}
            // Bit 3 {No Wrap (0) | Wrap (1)}
            // Bit 4 {Linear (0) | Non Linear (1)}
            // Bit 5 {Preferred State (0) | No Preferred (1)}
            // Bit 6 {No Null position (0) | Null state(1)}
            // Bit 7 {Non Volatile (0) | Volatile (1)}
            // Bit 8 {Bit Field (0) | Buffered Bytes (1)}
            // Bit 31- 9Reserved (0)
            MainOutputData {
                item_type: match first_byte & 0b00000001 {
                    0 => ItemType::Data,
                    _ => ItemType::Constant,
                },
                data_type: match (first_byte & 0b00000010) >> 1 {
                    0 => DataType::Array,
                    _ => DataType::Variable,
                },
                data_point: match (first_byte & 0b00000100) >> 2 {
                    0 => DataPoint::Absolute,
                    _ => DataPoint::Relative,
                },
                wrapping: match (first_byte & 0b00001000) >> 3 {
                    0 => Wrapping::NoWrap,
                    _ => Wrapping::Wrap,
                },
                linearity: match (first_byte & 0b00010000) >> 4 {
                    0 => Linearity::Linear,
                    _ => Linearity::Nonlinear,
                },
                state_preferrence: match (first_byte & 0b00100000) >> 5 {
                    0 => StatePreference::PreferredState,
                    _ => StatePreference::NoPreferredState,
                },
                null_state: match (first_byte & 0b01000000) >> 6 {
                    0 => NullState::NoNullState,
                    _ => NullState::NullState,
                },
                volatility: match (first_byte & 0b10000000) >> 7 {
                    0 => Volatility::NonVolatile,
                    _ => Volatility::Volatile,
                },
                field_type: match second_byte & 0b00000001 {
                    0 => FieldType::BitField,
                    _ => FieldType::BufferedBytes,
                },
            }
        }
    
        fn get_main_collection_type(&self, data_bits: &Vec<u8>) -> CollectionType {
            let first_byte = data_bits[0];
            match first_byte {
                0x00 => CollectionType::Physical,
                0x01 => CollectionType::Application,
                0x02 => CollectionType::Logical,
                0x03 => CollectionType::Report,
                0x04 => CollectionType::NamedArray,
                0x05 => CollectionType::UsageSwitch,
                0x06 => CollectionType::UsageModifier,
                _ => CollectionType::Reserved,
            }
        }
    
        fn get_global_tag(&self, item_builder: &HidItemBuilder) -> HidGlobalTag {
            // Extract bits 4 - 7, e.g. 0b01010101
            //                            ^^^^
            let tag = (item_builder.prefix_bits & 0b11110000) >> 4;
            return match tag {
                0b0000 => HidGlobalTag::UsagePage(self.get_u32(&item_builder.data_bits)),
                0b0001 => HidGlobalTag::LogicalMinimum(self.get_i32(&item_builder.data_bits)),
                0b0010 => HidGlobalTag::LogicalMaximum(self.get_i32(&item_builder.data_bits)),
                0b0011 => HidGlobalTag::PhysicalMinimum(self.get_i32(&item_builder.data_bits)),
                0b0100 => HidGlobalTag::PhysicalMaximum(self.get_i32(&item_builder.data_bits)),
                0b0101 => HidGlobalTag::UnitExponent(self.get_i32(&item_builder.data_bits)),
                0b0110 => HidGlobalTag::Unit(self.get_i32(&item_builder.data_bits)),
                0b0111 => HidGlobalTag::ReportSize(self.get_u32(&item_builder.data_bits)),
                0b1000 => HidGlobalTag::ReportId(self.get_u32(&item_builder.data_bits)),
                0b1001 => HidGlobalTag::ReportCount(self.get_u32(&item_builder.data_bits)),
                0b1010 => HidGlobalTag::Push(self.get_i32(&item_builder.data_bits)),
                0b1011 => HidGlobalTag::Pop(self.get_i32(&item_builder.data_bits)),
                0b1100..=0b1111 => HidGlobalTag::Reserved(self.get_i32(&item_builder.data_bits)),
                _ => todo!("How to handle illegal values?"),
            };
        }
    
        fn get_u32(&self, data_bits: &Vec<u8>) -> u32 {
            let mut value : u32 = 0;
    
            for (i, data) in data_bits.iter().enumerate() {
                let d = *data;
                value = value | ((d as u32) << (i * 8))
            }
    
            value
        }
    
        fn get_i32(&self, data_bits: &Vec<u8>) -> i32 {
            let mut value : i32 = 0;
    
            for (i, data) in data_bits.iter().enumerate() {
                let d = *data;
                value = value | ((d as i32) << (i * 8))
            }
    
            value
        }
    
        fn get_local_tag(&self, item_builder: &HidItemBuilder) -> HidLocalTag {
            // Extract bits 4 - 7, e.g. 0b01010101
            //                            ^^^^
            let tag = (item_builder.prefix_bits & 0b11110000) >> 4;
            return match tag {
                0b0000 => HidLocalTag::Usage(self.get_u32(&item_builder.data_bits)),
                0b0001 => HidLocalTag::UsageMinimum(self.get_u32(&item_builder.data_bits)),
                0b0010 => HidLocalTag::UsageMaximum(self.get_u32(&item_builder.data_bits)),
                0b0011 => HidLocalTag::DesignatorIndex(self.get_u32(&item_builder.data_bits)),
                0b0100 => HidLocalTag::DesignatorMinimum(self.get_u32(&item_builder.data_bits)),
                0b0101 => HidLocalTag::DesignatorMaximum(self.get_u32(&item_builder.data_bits)),
                // 0b0110 not defined
                0b0111 => HidLocalTag::StringIndex(self.get_u32(&item_builder.data_bits)),
                0b1000 => HidLocalTag::StringMinimum(self.get_u32(&item_builder.data_bits)),
                0b1001 => HidLocalTag::StringMaximum(self.get_u32(&item_builder.data_bits)),
                0b1010 => HidLocalTag::Delimiter(self.get_u32(&item_builder.data_bits)),
                0b1011..=0b1111 => HidLocalTag::Reserved(self.get_u32(&item_builder.data_bits)),
                _ => todo!("How to handle illegal values?"),
            };
        }
    }

    pub fn get_descriptor_report(bytes: &[u8]) -> Report {
        let mut report = Report::new();

        let mut i = 0;
        let mut collection_index = 0;
        let mut usage_pages: HashMap<i32, u32> = HashMap::new();
        while i < bytes.len() {
            let prefix_bits = bytes[i];

            let usage_page = match usage_pages.get(&collection_index) {
                Some(val) => Some(val.clone()),
                None => None,
            };
            let mut item_builder = HidItemBuilder::new(usage_page, prefix_bits);
            for _ in 0..item_builder.get_size() {
                i += 1;
                let item_data = bytes[i];
                item_builder.push_data(item_data);
            }
            report.items.push(item_builder.build());
            i += 1;

            let item = item_builder.build();
            match &item.global_tag {
                Some(tag) => {
                    match tag {
                        HidGlobalTag::UsagePage(up) => {
                            usage_pages.insert(collection_index, up.clone());
                        },
                        _ => {},
                    }
                }
                _ => {},
            }
            match &item.main_tag {
                Some(tag) => {
                    match tag {
                        HidMainTag::EndCollection => {
                            usage_pages.remove(&collection_index);
                            collection_index -= 1;
                        },
                        _ => {},
                    }
                },
                None => {},
            }

            match &item.main_tag {
                Some(tag) => {
                    match tag {
                        HidMainTag::Collection(_) => collection_index += 1,
                        _ => {},
                    }
                },
                None => {},
            }
        }

        report
    }
}

pub mod name {
    pub fn get_usage_page_name(usage_page: u32) -> String {
        match usage_page {
            0x01 => "Generic Desktop Page".to_string(),
            0x02 => "Simulation Controls Page".to_string(),
            0x03 => "VR Controls Page".to_string(),
            0x04 => "Sport Controls Page".to_string(),
            0x05 => "Game Controls Page".to_string(),
            0x06 => "Generic Device Controls Page".to_string(),
            0x07 => "Keyboard/Keypad Page".to_string(),
            0x08 => "LED Page".to_string(),
            0x09 => "Button Page".to_string(),
            0x0A => "Ordinal Page".to_string(),
            0x0B => "Telephony Page".to_string(),
            0x0C => "Consumer Page".to_string(),
            0x0D => "Digitizers Page".to_string(),
            0x0E => "Haptics Page".to_string(),
            0x0F => "Physical Interface Device Page".to_string(),
            0x10 => "Unicode Page".to_string(),
            0x12 => "Eye and Head Trackers Page".to_string(),
            0x14 => "Auxiliary Display Page".to_string(),
            0x20 => "Sensors Page".to_string(),
            0x40 => "Medical Instrument Page".to_string(),
            0x41 => "Braille Display Page".to_string(),
            0x59 => "Lighting And Illumination Page".to_string(),
            0x80..=0x83 => "Monitor Pages".to_string(),
            0x84..=0x87 => "Power Pages".to_string(),
            0x8C => "Bar Code Scanner Page".to_string(),
            0x8D => "Scale Page".to_string(),
            0x8E => "Magnetic Stripe Reading (MSR) Devices Page".to_string(),
            0x8F => "Point of Sale Pages".to_string(),
            0x90 => "Camera Control Page".to_string(),
            0x91 => "Arcade Page".to_string(),
            0x92 => "Gaming Device Page".to_string(),
            0xF1D0 => "FIDO Alliance Page".to_string(),
            0xFF00..=0xFFFF => format!("Vendor-defined {:#04X}", usage_page),
            _ => format!("{:#04X}", usage_page),
        }
    }

    pub fn get_usage_name(usage_page: Option<u32>, usage: u32) -> String {
        match usage_page {
            Some(value) => {
                match value {
                    0x01 => get_generic_desktop_usage_name(usage),
                    0x02 => get_simulation_controls_usage_name(usage),
                    0x03 => get_vr_controls_usage_name(usage),
                    0x04 => get_sport_controls_usage_name(usage),
                    0x05 => get_game_controls_usage_name(usage),
                    0x06 => get_generic_device_controls_usage_name(usage),
                    0x07 => get_keyboard_keypad_usage_name(usage),
                    0x08 => get_led_usage_name(usage),
                    0x09 => get_button_usage_name(usage),
                    0x0A => get_ordinal_usage_name(usage),
                    0x0B => get_telephony_usage_name(usage),
                    0x0C => get_consumer_usage_name(usage),
                    0x0D => get_digitizers_usage_name(usage),
                    0x0E => get_haptics_usage_name(usage),
                    0x0F => get_physical_interface_device_usage_name(usage),
                    0x10 => get_unicode_usage_name(usage),
                    0x12 => get_eye_and_head_trackers_usage_name(usage),
                    0x14 => get_auxiliary_display_usage_name(usage),
                    0x20 => get_sensors_usage_name(usage),
                    0x40 => get_medical_instrument_usage_name(usage),
                    0x41 => get_braille_display_usage_name(usage),
                    0x59 => get_lightning_and_illumination_usage_name(usage),
                    0x80..=0x83 => get_monitor_usage_name(value, usage),
                    0x84..=0x87 => get_power_usage_name(value, usage),
                    0x8C => get_bar_code_scanner_usage_name(usage),
                    0x8D => get_scale_usage_name(usage),
                    0x8E => get_magnetic_stripe_reading_devices_usage_name(usage),
                    0x8F => get_point_of_sale_usage_name(usage),
                    0x90 => get_camera_control_usage_name(usage),
                    0x91 => get_arcade_usage_name(usage),
                    0x92 => get_gaming_device_usage_name(usage),
                    0xF1D0 => get_fido_alliance_usage_name(usage),
                    0xFF00..=0xFFFF => get_vendor_defined_usage_name(value, usage),
                    _ => format!("{:#04X}", usage),
                }
            },
            None => format!("{:#04X}", usage),
        }
    }

    fn get_generic_desktop_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Pointer ".to_string(),
            0x02 => "Mouse ".to_string(),
            0x03..=0x03 => "Reserved".to_string(),
            0x04 => "Joystick ".to_string(),
            0x05 => "Gamepad ".to_string(),
            0x06 => "Keyboard ".to_string(),
            0x07 => "Keypad ".to_string(),
            0x08 => "Multi-axis Controller ".to_string(),
            0x09 => "Tablet PC System Controls ".to_string(),
            0x0A => "Water Cooling Device ".to_string(),
            0x0B => "Computer Chassis Device ".to_string(),
            0x0C => "Wireless Radio Controls ".to_string(),
            0x0D => "Portable Device Control ".to_string(),
            0x0E => "System Multi-Axis Controller ".to_string(),
            0x0F => "Spatial Controller ".to_string(),
            0x10 => "Assistive Control ".to_string(),
            0x11 => "Device Dock ".to_string(),
            0x12 => "Dockable Device ".to_string(),
            0x13..=0x2F => "Reserved".to_string(),
            0x30 => "X ".to_string(),
            0x31 => "Y ".to_string(),
            0x32 => "Z ".to_string(),
            0x33 => "Rx ".to_string(),
            0x34 => "Ry ".to_string(),
            0x35 => "Rz ".to_string(),
            0x36 => "Slider ".to_string(),
            0x37 => "Dial ".to_string(),
            0x38 => "Wheel ".to_string(),
            0x39 => "Hat Switch ".to_string(),
            0x3A => "Counted Buffer ".to_string(),
            0x3B => "Byte Count ".to_string(),
            0x3C => "Motion Wakeup ".to_string(),
            0x3D => "Start ".to_string(),
            0x3E => "Select ".to_string(),
            0x3F..=0x3F => "Reserved".to_string(),
            0x40 => "Vx ".to_string(),
            0x41 => "Vy ".to_string(),
            0x42 => "Vz ".to_string(),
            0x43 => "Vbrx ".to_string(),
            0x44 => "Vbry ".to_string(),
            0x45 => "Vbrz ".to_string(),
            0x46 => "Vno ".to_string(),
            0x47 => "Feature Notification ".to_string(),
            0x48 => "Resolution Multiplier ".to_string(),
            0x49 => "Qx ".to_string(),
            0x4A => "Qy ".to_string(),
            0x4B => "Qz ".to_string(),
            0x4C => "Qw ".to_string(),
            0x4D..=0x7F => "Reserved".to_string(),
            0x80 => "System Control ".to_string(),
            0x81 => "System Power Down ".to_string(),
            0x82 => "System Sleep ".to_string(),
            0x83 => "System Wake Up ".to_string(),
            0x84 => "System Context Menu ".to_string(),
            0x85 => "System Main Menu ".to_string(),
            0x86 => "System App Menu ".to_string(),
            0x87 => "System Menu Help ".to_string(),
            0x88 => "System Menu Exit ".to_string(),
            0x89 => "System Menu Select ".to_string(),
            0x8A => "System Menu Right ".to_string(),
            0x8B => "System Menu Left ".to_string(),
            0x8C => "System Menu Up ".to_string(),
            0x8D => "System Menu Down ".to_string(),
            0x8E => "System Cold Restart ".to_string(),
            0x8F => "System Warm Restart ".to_string(),
            0x90 => "D-pad Up ".to_string(),
            0x91 => "D-pad Down ".to_string(),
            0x92 => "D-pad Right ".to_string(),
            0x93 => "D-pad Left ".to_string(),
            0x94 => "Index Trigger ".to_string(),
            0x95 => "Palm Trigger ".to_string(),
            0x96 => "Thumbstick ".to_string(),
            0x97 => "System Function Shift ".to_string(),
            0x98 => "System Function Shift Lock ".to_string(),
            0x99 => "System Function Shift Lock Indicator ".to_string(),
            0x9A => "System Dismiss Notification ".to_string(),
            0x9B => "System Do Not Disturb ".to_string(),
            0x9C..=0x9F => "Reserved".to_string(),
            0xA0 => "System Dock ".to_string(),
            0xA1 => "System Undock ".to_string(),
            0xA2 => "System Setup ".to_string(),
            0xA3 => "System Break ".to_string(),
            0xA4 => "System Debugger Break ".to_string(),
            0xA5 => "Application Break ".to_string(),
            0xA6 => "Application Debugger Break ".to_string(),
            0xA7 => "System Speaker Mute ".to_string(),
            0xA8 => "System Hibernate ".to_string(),
            0xA9..=0xAF => "Reserved".to_string(),
            0xB0 => "System Display Invert ".to_string(),
            0xB1 => "System Display Internal ".to_string(),
            0xB2 => "System Display External ".to_string(),
            0xB3 => "System Display Both ".to_string(),
            0xB4 => "System Display Dual ".to_string(),
            0xB5 => "System Display Toggle Int/Ext Mode ".to_string(),
            0xB6 => "System Display Swap Primary/Secondary ".to_string(),
            0xB7 => "System Display Toggle LCD Autoscale ".to_string(),
            0xB8..=0xBF => "Reserved".to_string(),
            0xC0 => "Sensor Zone ".to_string(),
            0xC1 => "RPM ".to_string(),
            0xC2 => "Coolant Level ".to_string(),
            0xC3 => "Coolant Critical Level ".to_string(),
            0xC4 => "Coolant Pump ".to_string(),
            0xC5 => "Chassis Enclosure ".to_string(),
            0xC6 => "Wireless Radio Button ".to_string(),
            0xC7 => "Wireless Radio LED ".to_string(),
            0xC8 => "Wireless Radio Slider Switch ".to_string(),
            0xC9 => "System Display Rotation Lock Button ".to_string(),
            0xCA => "System Display Rotation Lock Slider Switch ".to_string(),
            0xCB => "Control Enable ".to_string(),
            0xCC..=0xCF => "Reserved".to_string(),
            0xD0 => "Dockable Device Unique ID ".to_string(),
            0xD1 => "Dockable Device Vendor ID ".to_string(),
            0xD2 => "Dockable Device Primary Usage Page ".to_string(),
            0xD3 => "Dockable Device Primary Usage ID ".to_string(),
            0xD4 => "Dockable Device Docking State ".to_string(),
            0xD5 => "Dockable Device Display Occlusion ".to_string(),
            0xD6 => "Dockable Device Object Type ".to_string(),
            0xD7..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_simulation_controls_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Flight Simulation Device".to_string(),
            0x02 => "Automobile Simulation Device".to_string(),
            0x03 => "Tank Simulation Device".to_string(),
            0x04 => "Spaceship Simulation Device".to_string(),
            0x05 => "Submarine Simulation Device".to_string(),
            0x06 => "Sailing Simulation Device".to_string(),
            0x07 => "Motorcycle Simulation Device".to_string(),
            0x08 => "Sports Simulation Device".to_string(),
            0x09 => "Airplane Simulation Device".to_string(),
            0x0A => "Helicopter Simulation Device".to_string(),
            0x0B => "Magic Carpet Simulation Device".to_string(),
            0x0C => "Bicycle Simulation Device".to_string(),
            0x0D..=0x1F => "Reserved".to_string(),
            0x20 => "Flight Control Stick".to_string(),
            0x21 => "Flight Stick".to_string(),
            0x22 => "Cyclic Control".to_string(),
            0x23 => "Cyclic Trim".to_string(),
            0x24 => "Flight Yoke".to_string(),
            0x25 => "Track Control".to_string(),
            0x26..=0xAF => "Reserved".to_string(),
            0xB0 => "Aileron".to_string(),
            0xB1 => "Aileron Trim".to_string(),
            0xB2 => "Anti-Torque Control".to_string(),
            0xB3 => "Autopilot Enable".to_string(),
            0xB4 => "Chaff Release".to_string(),
            0xB5 => "Collective Control".to_string(),
            0xB6 => "Dive Brake".to_string(),
            0xB7 => "Electronic Countermeasures".to_string(),
            0xB8 => "Elevator".to_string(),
            0xB9 => "Elevator Trim".to_string(),
            0xBA => "Rudder".to_string(),
            0xBB => "Throttle".to_string(),
            0xBC => "Flight Communications".to_string(),
            0xBD => "Flare Release".to_string(),
            0xBE => "Landing Gear".to_string(),
            0xBF => "Toe Brake".to_string(),
            0xC0 => "Trigger".to_string(),
            0xC1 => "Weapons Arm".to_string(),
            0xC2 => "Weapons Select".to_string(),
            0xC3 => "Wing Flaps".to_string(),
            0xC4 => "Accelerator".to_string(),
            0xC5 => "Brake".to_string(),
            0xC6 => "Clutch".to_string(),
            0xC7 => "Shifter".to_string(),
            0xC8 => "Steering".to_string(),
            0xC9 => "Turret Direction".to_string(),
            0xCA => "Barrel Elevation".to_string(),
            0xCB => "Dive Plane".to_string(),
            0xCC => "Ballast".to_string(),
            0xCD => "Bicycle Crank".to_string(),
            0xCE => "Handle Bars".to_string(),
            0xCF => "Front Brake".to_string(),
            0xD0 => "Rear Brake".to_string(),
            0xD1..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_vr_controls_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Belt".to_string(),
            0x02 => "Body Suit".to_string(),
            0x03 => "Flexor".to_string(),
            0x04 => "Glove".to_string(),
            0x05 => "Head Tracker".to_string(),
            0x06 => "Head Mounted Display".to_string(),
            0x07 => "Hand Tracker".to_string(),
            0x08 => "Oculometer".to_string(),
            0x09 => "Vest".to_string(),
            0x0A => "Animatronic Device".to_string(),
            0x0B..=0x1F => "Reserved".to_string(),
            0x20 => "Stereo Enable".to_string(),
            0x21 => "Display Enable".to_string(),
            0x22..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_sport_controls_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Baseball Bat".to_string(),
            0x02 => "Golf Club".to_string(),
            0x03 => "Rowing Machine".to_string(),
            0x04 => "Treadmill".to_string(),
            0x05..=0x2F => "Reserved".to_string(),
            0x30 => "Oar".to_string(),
            0x31 => "Slope".to_string(),
            0x32 => "Rate".to_string(),
            0x33 => "Stick Speed".to_string(),
            0x34 => "Stick Face Angle".to_string(),
            0x35 => "Stick Heel/Toe".to_string(),
            0x36 => "Stick Follow Through".to_string(),
            0x37 => "Stick Tempo".to_string(),
            0x38 => "Stick Type NAry 7.1".to_string(),
            0x39 => "Stick Height".to_string(),
            0x3A..=0x4F => "Reserved".to_string(),
            0x50 => "Putter Sel 7.1".to_string(),
            0x51 => "1 Iron Sel 7.1".to_string(),
            0x52 => "2 Iron Sel 7.1".to_string(),
            0x53 => "3 Iron Sel 7.1".to_string(),
            0x54 => "4 Iron Sel 7.1".to_string(),
            0x55 => "5 Iron Sel 7.1".to_string(),
            0x56 => "6 Iron Sel 7.1".to_string(),
            0x57 => "7 Iron Sel 7.1".to_string(),
            0x58 => "8 Iron Sel 7.1".to_string(),
            0x59 => "9 Iron Sel 7.1".to_string(),
            0x5A => "10 Iron Sel 7.1".to_string(),
            0x5B => "11 Iron Sel 7.1".to_string(),
            0x5C => "Sand Wedge Sel 7.1".to_string(),
            0x5D => "Loft Wedge Sel 7.1".to_string(),
            0x5E => "Power Wedge Sel 7.1".to_string(),
            0x5F => "1 Wood Sel 7.1".to_string(),
            0x60 => "3 Wood Sel 7.1".to_string(),
            0x61 => "5 Wood Sel 7.1".to_string(),
            0x62 => "7 Wood Sel 7.1".to_string(),
            0x63 => "9 Wood Sel 7.1".to_string(),
            0x64..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_game_controls_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "3D Game Controller".to_string(),
            0x02 => "Pinball Device".to_string(),
            0x03 => "Gun Device".to_string(),
            0x04..=0x1F => "Reserved".to_string(),
            0x20 => "Point of View".to_string(),
            0x21 => "Turn Right/Left".to_string(),
            0x22 => "Pitch Forward/Backward".to_string(),
            0x23 => "Roll Right/Left".to_string(),
            0x24 => "Move Right/Left".to_string(),
            0x25 => "Move Forward/Backward".to_string(),
            0x26 => "Move Up/Down".to_string(),
            0x27 => "Lean Right/Left".to_string(),
            0x28 => "Lean Forward/Backward".to_string(),
            0x29 => "Height of POV".to_string(),
            0x2A => "Flipper".to_string(),
            0x2B => "Secondary Flipper".to_string(),
            0x2C => "Bump".to_string(),
            0x2D => "New Game".to_string(),
            0x2E => "Shoot Ball".to_string(),
            0x2F => "Player".to_string(),
            0x30 => "Gun Bolt".to_string(),
            0x31 => "Gun Clip".to_string(),
            0x32 => "Gun Selector NAry 8.3".to_string(),
            0x33 => "Gun Single Shot Sel 8.3".to_string(),
            0x34 => "Gun Burst Sel 8.3".to_string(),
            0x35 => "Gun Automatic Sel 8.3".to_string(),
            0x36 => "Gun Safety".to_string(),
            0x37 => "Gamepad Fire/Jump CL 8.4.1".to_string(),
            0x38..=0x38 => "Reserved".to_string(),
            0x39 => "Gamepad Trigger CL 8.4.1".to_string(),
            0x3A => "Form-fitting Gamepad SF 8.4.1".to_string(),
            0x3B..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_generic_device_controls_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Background/Nonuser Controls [4] [65]".to_string(),
            0x02..=0x1F => "Reserved".to_string(),
            0x20 => "Battery Strength".to_string(),
            0x21 => "Wireless Channel".to_string(),
            0x22 => "Wireless ID".to_string(),
            0x23 => "Discover Wireless Control".to_string(),
            0x24 => "Security Code Character Entered".to_string(),
            0x25 => "Security Code Character Erased".to_string(),
            0x26 => "Security Code Cleared".to_string(),
            0x27 => "Sequence ID [5]".to_string(),
            0x28 => "Sequence ID Reset [5]".to_string(),
            0x29 => "RF Signal Strength [5]".to_string(),
            0x2A => "Software Version [32]".to_string(),
            0x2B => "Protocol Version [32]".to_string(),
            0x2C => "Hardware Version [32]".to_string(),
            0x2D => "Major [32]".to_string(),
            0x2E => "Minor [32]".to_string(),
            0x2F => "Revision [32]".to_string(),
            0x30 => "Handedness [40] NAry 9.4".to_string(),
            0x31 => "Either Hand [40] Sel 9.4".to_string(),
            0x32 => "Left Hand [40] Sel 9.4".to_string(),
            0x33 => "Right Hand [40] Sel 9.4".to_string(),
            0x34 => "Both Hands [40] Sel 9.4".to_string(),
            0x35..=0x3F => "Reserved".to_string(),
            0x40 => "Grip Pose Offset [40]".to_string(),
            0x41 => "Pointer Pose Offset [40]".to_string(),
            0x42..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_keyboard_keypad_usage_name(usage: u32) -> String {
        match usage {
            0x00..=0x00 => "Reserved".to_string(),
            0x01 => "Keyboard ErrorRollOver".to_string(),
            0x02 => "Keyboard POSTFail".to_string(),
            0x03 => "Keyboard ErrorUndefined".to_string(),
            0x04 => "Keyboard a and A".to_string(),
            0x05 => "Keyboard b and B".to_string(),
            0x06 => "Keyboard c and C".to_string(),
            0x07 => "Keyboard d and D".to_string(),
            0x08 => "Keyboard e and E".to_string(),
            0x09 => "Keyboard f and F".to_string(),
            0x0A => "Keyboard g and G".to_string(),
            0x0B => "Keyboard h and H".to_string(),
            0x0C => "Keyboard i and I".to_string(),
            0x0D => "Keyboard j and J".to_string(),
            0x0E => "Keyboard k and K".to_string(),
            0x0F => "Keyboard l and L".to_string(),
            0x10 => "Keyboard m and M".to_string(),
            0x11 => "Keyboard n and N".to_string(),
            0x12 => "Keyboard o and O".to_string(),
            0x13 => "Keyboard p and P".to_string(),
            0x14 => "Keyboard q and Q".to_string(),
            0x15 => "Keyboard r and R".to_string(),
            0x16 => "Keyboard s and S".to_string(),
            0x17 => "Keyboard t and T".to_string(),
            0x18 => "Keyboard u and U".to_string(),
            0x19 => "Keyboard v and V".to_string(),
            0x1A => "Keyboard w and W".to_string(),
            0x1B => "Keyboard x and X".to_string(),
            0x1C => "Keyboard y and Y".to_string(),
            0x1D => "Keyboard z and Z".to_string(),
            0x1E => "Keyboard 1 and !".to_string(),
            0x1F => "Keyboard 2 and @".to_string(),
            0x20 => "Keyboard 3 and #".to_string(),
            0x21 => "Keyboard 4 and $".to_string(),
            0x22 => "Keyboard 5 and %".to_string(),
            0x23 => "Keyboard 6 and ∧".to_string(),
            0x24 => "Keyboard 7 and &".to_string(),
            0x25 => "Keyboard 8 and *".to_string(),
            0x26 => "Keyboard 9 and (".to_string(),
            0x27 => "Keyboard 0 and )".to_string(),
            0x28 => "Keyboard Return (ENTER)".to_string(),
            0x29 => "Keyboard ESCAPE".to_string(),
            0x2A => "Keyboard DELETE (Backspace)".to_string(),
            0x2B => "Keyboard Tab".to_string(),
            0x2C => "Keyboard Spacebar".to_string(),
            0x2D => "Keyboard - and (underscore)".to_string(),
            0x2E => "Keyboard = and +".to_string(),
            0x2F => "Keyboard [ and {".to_string(),
            0x30 => "Keyboard ] and }".to_string(),
            0x31 => "Keyboard \\ and |".to_string(),
            0x32 => "Keyboard Non-US # and ̃".to_string(),
            0x33 => "Keyboard ; and :".to_string(),
            0x34 => "Keyboard \u{2018} and “".to_string(),
            0x35 => "Keyboard Grave Accent and Tilde".to_string(),
            0x36 => "Keyboard , and <".to_string(),
            0x37 => "Keyboard . and >".to_string(),
            0x38 => "Keyboard / and ?".to_string(),
            0x39 => "Keyboard Caps Lock".to_string(),
            0x3A => "Keyboard F1".to_string(),
            0x3B => "Keyboard F2".to_string(),
            0x3C => "Keyboard F3".to_string(),
            0x3D => "Keyboard F4".to_string(),
            0x3E => "Keyboard F5".to_string(),
            0x3F => "Keyboard F6".to_string(),
            0x40 => "Keyboard F7".to_string(),
            0x41 => "Keyboard F8".to_string(),
            0x42 => "Keyboard F9".to_string(),
            0x43 => "Keyboard F10".to_string(),
            0x44 => "Keyboard F11".to_string(),
            0x45 => "Keyboard F12".to_string(),
            0x46 => "Keyboard PrintScreen".to_string(),
            0x47 => "Keyboard Scroll Lock".to_string(),
            0x48 => "Keyboard Pause".to_string(),
            0x49 => "Keyboard Insert".to_string(),
            0x4A => "Keyboard Home".to_string(),
            0x4B => "Keyboard PageUp".to_string(),
            0x4C => "Keyboard Delete Forward".to_string(),
            0x4D => "Keyboard End".to_string(),
            0x4E => "Keyboard PageDown".to_string(),
            0x4F => "Keyboard RightArrow".to_string(),
            0x50 => "Keyboard LeftArrow".to_string(),
            0x51 => "Keyboard DownArrow".to_string(),
            0x52 => "Keyboard UpArrow".to_string(),
            0x53 => "Keypad Num Lock and Clear".to_string(),
            0x54 => "Keypad /".to_string(),
            0x55 => "Keypad *".to_string(),
            0x56 => "Keypad -".to_string(),
            0x57 => "Keypad +".to_string(),
            0x58 => "Keypad ENTER".to_string(),
            0x59 => "Keypad 1 and End".to_string(),
            0x5A => "Keypad 2 and Down Arrow".to_string(),
            0x5B => "Keypad 3 and PageDn".to_string(),
            0x5C => "Keypad 4 and Left Arrow".to_string(),
            0x5D => "Keypad 5".to_string(),
            0x5E => "Keypad 6 and Right Arrow".to_string(),
            0x5F => "Keypad 7 and Home".to_string(),
            0x60 => "Keypad 8 and Up Arrow".to_string(),
            0x61 => "Keypad 9 and PageUp".to_string(),
            0x62 => "Keypad 0 and Insert".to_string(),
            0x63 => "Keypad . and Delete".to_string(),
            0x64 => "Keyboard Non-US \\ and |".to_string(),
            0x65 => "Keyboard Application".to_string(),
            0x66 => "Keyboard Power".to_string(),
            0x67 => "Keypad =".to_string(),
            0x68 => "Keyboard F13".to_string(),
            0x69 => "Keyboard F14".to_string(),
            0x6A => "Keyboard F15".to_string(),
            0x6B => "Keyboard F16".to_string(),
            0x6C => "Keyboard F17".to_string(),
            0x6D => "Keyboard F18".to_string(),
            0x6E => "Keyboard F19".to_string(),
            0x6F => "Keyboard F20".to_string(),
            0x70 => "Keyboard F21".to_string(),
            0x71 => "Keyboard F22".to_string(),
            0x72 => "Keyboard F23".to_string(),
            0x73 => "Keyboard F24".to_string(),
            0x74 => "Keyboard Execute".to_string(),
            0x75 => "Keyboard Help".to_string(),
            0x76 => "Keyboard Menu".to_string(),
            0x77 => "Keyboard".to_string(),
            0x78 => "Keyboard Stop".to_string(),
            0x79 => "Keyboard Again".to_string(),
            0x7A => "Keyboard Undo".to_string(),
            0x7B => "Keyboard Cut".to_string(),
            0x7C => "Keyboard Copy".to_string(),
            0x7D => "Keyboard Paste".to_string(),
            0x7E => "Keyboard Find".to_string(),
            0x7F => "Keyboard Mute".to_string(),
            0x80 => "Keyboard Volume Up".to_string(),
            0x81 => "Keyboard Volume Down".to_string(),
            0x82 => "Keyboard Locking Caps Lock".to_string(),
            0x83 => "Keyboard Locking Num Lock".to_string(),
            0x84 => "Keyboard Locking Scroll Lock".to_string(),
            0x85 => "Keypad Comma".to_string(),
            0x86 => "Keypad Equal Sign".to_string(),
            0x87 => "Keyboard International1".to_string(),
            0x88 => "Keyboard International2".to_string(),
            0x89 => "Keyboard International3".to_string(),
            0x8A => "Keyboard International4".to_string(),
            0x8B => "Keyboard International5".to_string(),
            0x8C => "Keyboard International6".to_string(),
            0x8D => "Keyboard International7".to_string(),
            0x8E => "Keyboard International8".to_string(),
            0x8F => "Keyboard International9".to_string(),
            0x90 => "Keyboard LANG1".to_string(),
            0x91 => "Keyboard LANG2".to_string(),
            0x92 => "Keyboard LANG3".to_string(),
            0x93 => "Keyboard LANG4".to_string(),
            0x94 => "Keyboard LANG5".to_string(),
            0x95 => "Keyboard LANG6".to_string(),
            0x96 => "Keyboard LANG7".to_string(),
            0x97 => "Keyboard LANG8".to_string(),
            0x98 => "Keyboard LANG9".to_string(),
            0x99 => "Keyboard Alternate Erase".to_string(),
            0x9A => "Keyboard SysReq/Attention".to_string(),
            0x9B => "Keyboard Cancel".to_string(),
            0x9C => "Keyboard Clear".to_string(),
            0x9D => "Keyboard Prior".to_string(),
            0x9E => "Keyboard Return".to_string(),
            0x9F => "Keyboard Separator".to_string(),
            0xA0 => "Keyboard Out".to_string(),
            0xA1 => "Keyboard Oper".to_string(),
            0xA2 => "Keyboard Clear/Again".to_string(),
            0xA3 => "Keyboard CrSel/Props".to_string(),
            0xA4 => "Keyboard ExSel".to_string(),
            0xA5..=0xAF => "Reserved".to_string(),
            0xB0 => "Keypad 00".to_string(),
            0xB1 => "Keypad 000".to_string(),
            0xB2 => "Thousands Separator".to_string(),
            0xB3 => "Decimal Separator".to_string(),
            0xB4 => "Currency Unit".to_string(),
            0xB5 => "Currency Sub-unit".to_string(),
            0xB6 => "Keypad (".to_string(),
            0xB7 => "Keypad )".to_string(),
            0xB8 => "Keypad {".to_string(),
            0xB9 => "Keypad }".to_string(),
            0xBA => "Keypad Tab".to_string(),
            0xBB => "Keypad Backspace".to_string(),
            0xBC => "Keypad A".to_string(),
            0xBD => "Keypad B".to_string(),
            0xBE => "Keypad C".to_string(),
            0xBF => "Keypad D".to_string(),
            0xC0 => "Keypad E".to_string(),
            0xC1 => "Keypad F".to_string(),
            0xC2 => "Keypad XOR".to_string(),
            0xC3 => "Keypad ∧".to_string(),
            0xC4 => "Keypad %".to_string(),
            0xC5 => "Keypad <".to_string(),
            0xC6 => "Keypad >".to_string(),
            0xC7 => "Keypad &".to_string(),
            0xC8 => "Keypad &&".to_string(),
            0xC9 => "Keypad |".to_string(),
            0xCA => "Keypad ||".to_string(),
            0xCB => "Keypad :".to_string(),
            0xCC => "Keypad #".to_string(),
            0xCD => "Keypad Space".to_string(),
            0xCE => "Keypad @".to_string(),
            0xCF => "Keypad !".to_string(),
            0xD0 => "Keypad Memory Store".to_string(),
            0xD1 => "Keypad Memory Recall".to_string(),
            0xD2 => "Keypad Memory Clear".to_string(),
            0xD3 => "Keypad Memory Add".to_string(),
            0xD4 => "Keypad Memory Subtract".to_string(),
            0xD5 => "Keypad Memory Multiply".to_string(),
            0xD6 => "Keypad Memory Divide".to_string(),
            0xD7 => "Keypad +/-".to_string(),
            0xD8 => "Keypad Clear".to_string(),
            0xD9 => "Keypad Clear Entry".to_string(),
            0xDA => "Keypad Binary".to_string(),
            0xDB => "Keypad Octal".to_string(),
            0xDC => "Keypad Decimal".to_string(),
            0xDD => "Keypad Hexadecimal".to_string(),
            0xDE..=0xDF => "Reserved".to_string(),
            0xE0 => "Keyboard LeftControl".to_string(),
            0xE1 => "Keyboard LeftShift".to_string(),
            0xE2 => "Keyboard LeftAlt".to_string(),
            0xE3 => "Keyboard Left GUI".to_string(),
            0xE4 => "Keyboard RightControl".to_string(),
            0xE5 => "Keyboard RightShift".to_string(),
            0xE6 => "Keyboard RightAlt".to_string(),
            0xE7 => "Keyboard Right GUI".to_string(),
            0xE8..=0xFFFF => "Reserve".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_led_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Num Lock".to_string(),
            0x02 => "Caps Lock".to_string(),
            0x03 => "Scroll Lock".to_string(),
            0x04 => "Compose".to_string(),
            0x05 => "Kana".to_string(),
            0x06 => "Power".to_string(),
            0x07 => "Shift".to_string(),
            0x08 => "Do Not Disturb".to_string(),
            0x09 => "Mute".to_string(),
            0x0A => "Tone Enable".to_string(),
            0x0B => "High Cut Filter".to_string(),
            0x0C => "Low Cut Filter".to_string(),
            0x0D => "Equalizer Enable".to_string(),
            0x0E => "Sound Field On".to_string(),
            0x0F => "Surround On".to_string(),
            0x10 => "Repeat".to_string(),
            0x11 => "Stereo".to_string(),
            0x12 => "Sampling Rate Detect".to_string(),
            0x13 => "Spinning".to_string(),
            0x14 => "CAV".to_string(),
            0x15 => "CLV".to_string(),
            0x16 => "Recording Format Detect".to_string(),
            0x17 => "Off-Hook".to_string(),
            0x18 => "Ring".to_string(),
            0x19 => "Message Waiting".to_string(),
            0x1A => "Data Mode".to_string(),
            0x1B => "Battery Operation".to_string(),
            0x1C => "Battery OK".to_string(),
            0x1D => "Battery Low".to_string(),
            0x1E => "Speaker".to_string(),
            0x1F => "Head Set".to_string(),
            0x20 => "Hold".to_string(),
            0x21 => "Microphone".to_string(),
            0x22 => "Coverage".to_string(),
            0x23 => "Night Mode".to_string(),
            0x24 => "Send Calls".to_string(),
            0x25 => "Call Pickup".to_string(),
            0x26 => "Conference".to_string(),
            0x27 => "Stand-by".to_string(),
            0x28 => "Camera On".to_string(),
            0x29 => "Camera Off".to_string(),
            0x2A => "On-Line".to_string(),
            0x2B => "Off-Line".to_string(),
            0x2C => "Busy".to_string(),
            0x2D => "Ready".to_string(),
            0x2E => "Paper-Out".to_string(),
            0x2F => "Paper-Jam".to_string(),
            0x30 => "Remote".to_string(),
            0x31 => "Forward".to_string(),
            0x32 => "Reverse".to_string(),
            0x33 => "Stop".to_string(),
            0x34 => "Rewind".to_string(),
            0x35 => "Fast Forward".to_string(),
            0x36 => "Play".to_string(),
            0x37 => "Pause".to_string(),
            0x38 => "Record".to_string(),
            0x39 => "Error".to_string(),
            0x3A => "Usage Selected Indicator".to_string(),
            0x3B => "Usage In Use Indicator".to_string(),
            0x3C => "Usage Multi Mode Indicator".to_string(),
            0x3D => "Indicator On".to_string(),
            0x3E => "Indicator Flash".to_string(),
            0x3F => "Indicator Slow Blink".to_string(),
            0x40 => "Indicator Fast Blink".to_string(),
            0x41 => "Indicator Off".to_string(),
            0x42 => "Flash On Time".to_string(),
            0x43 => "Slow Blink On Time".to_string(),
            0x44 => "Slow Blink Off Time".to_string(),
            0x45 => "Fast Blink On Time".to_string(),
            0x46 => "Fast Blink Off Time".to_string(),
            0x47 => "Usage Indicator Color".to_string(),
            0x48 => "Indicator Red".to_string(),
            0x49 => "Indicator Green".to_string(),
            0x4A => "Indicator Amber".to_string(),
            0x4B => "Generic Indicator".to_string(),
            0x4C => "System Suspend".to_string(),
            0x4D => "External Power Connected".to_string(),
            0x4E => "Indicator Blue".to_string(),
            0x4F => "Indicator Orange".to_string(),
            0x50 => "Good Status".to_string(),
            0x51 => "Warning Status".to_string(),
            0x52 => "RGB LED".to_string(),
            0x53 => "Red LED Channel".to_string(),
            0x54 => "Blue LED Channel".to_string(),
            0x55 => "Green LED Channel".to_string(),
            0x56 => "LED Intensity".to_string(),
            0x57..=0x5F => "Reserved".to_string(),
            0x60 => "Player Indicator".to_string(),
            0x61 => "Player 1".to_string(),
            0x62 => "Player 2".to_string(),
            0x63 => "Player 3".to_string(),
            0x64 => "Player 4".to_string(),
            0x65 => "Player 5".to_string(),
            0x66 => "Player 6".to_string(),
            0x67 => "Player 7".to_string(),
            0x68 => "Player 8".to_string(),
            0x69..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_button_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "No Button Pressed".to_string(),
            0x01 => "Button 1 (primary/trigger)".to_string(),
            0x02 => "Button 2 (secondary)".to_string(),
            0x03 => "Button 3 (tertiary)".to_string(),
            0x04..=0xFFFF => format!("Button {}", usage),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_ordinal_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Reserved".to_string(),
            0x01..=0xFFFF => format!("Instance {}", usage),
            _ => format!("{:#04X}", usage),
        }
    }
    
    fn get_telephony_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Phone".to_string(),
            0x02 => "Answering Machine".to_string(),
            0x03 => "Message Controls".to_string(),
            0x04 => "Handset".to_string(),
            0x05 => "Headset".to_string(),
            0x06 => "Telephony Key Pad".to_string(),
            0x07 => "Programmable Button".to_string(),
            0x08..=0x1F => "Reserved".to_string(),
            0x20 => "Hook Switch".to_string(),
            0x21 => "Flash".to_string(),
            0x22 => "Feature".to_string(),
            0x23 => "Hold".to_string(),
            0x24 => "Redial".to_string(),
            0x25 => "Transfer".to_string(),
            0x26 => "Drop".to_string(),
            0x27 => "Park".to_string(),
            0x28 => "Forward Calls".to_string(),
            0x29 => "Alternate Function".to_string(),
            0x2A => "Line".to_string(),
            0x2B => "Speaker Phone".to_string(),
            0x2C => "Conference".to_string(),
            0x2D => "Ring Enable".to_string(),
            0x2E => "Ring Select".to_string(),
            0x2F => "Phone Mute".to_string(),
            0x30 => "Caller ID".to_string(),
            0x31 => "Send".to_string(),
            0x32..=0x4F => "Reserved".to_string(),
            0x50 => "Speed Dial".to_string(),
            0x51 => "Store Number".to_string(),
            0x52 => "Recall Number".to_string(),
            0x53 => "Phone Directory".to_string(),
            0x54..=0x6F => "Reserved".to_string(),
            0x70 => "Voice Mail".to_string(),
            0x71 => "Screen Calls".to_string(),
            0x72 => "Do Not Disturb".to_string(),
            0x73 => "Message".to_string(),
            0x74 => "Answer On/Off".to_string(),
            0x75..=0x8F => "Reserved".to_string(),
            0x90 => "Inside Dial Tone".to_string(),
            0x91 => "Outside Dial Tone".to_string(),
            0x92 => "Inside Ring Tone".to_string(),
            0x93 => "Outside Ring Tone".to_string(),
            0x94 => "Priority Ring Tone".to_string(),
            0x95 => "Inside Ringback".to_string(),
            0x96 => "Priority Ringback".to_string(),
            0x97 => "Line Busy Tone".to_string(),
            0x98 => "Reorder Tone".to_string(),
            0x99 => "Call Waiting Tone".to_string(),
            0x9A => "Confirmation Tone 1".to_string(),
            0x9B => "Confirmation Tone 1".to_string(),
            0x9C => "Tones Off".to_string(),
            0x9D => "Outside Ringback".to_string(),
            0x9E => "Ringer".to_string(),
            0x9F..=0xAF => "Reserved".to_string(),
            0xB0 => "Phone Key 0".to_string(),
            0xB1 => "Phone Key 1".to_string(),
            0xB2 => "Phone Key 2".to_string(),
            0xB3 => "Phone Key 3".to_string(),
            0xB4 => "Phone Key 4".to_string(),
            0xB5 => "Phone Key 5".to_string(),
            0xB6 => "Phone Key 6".to_string(),
            0xB7 => "Phone Key 7".to_string(),
            0xB8 => "Phone Key 8".to_string(),
            0xB9 => "Phone Key 9".to_string(),
            0xBA => "Phone Key Star".to_string(),
            0xBB => "Phone Key Pound".to_string(),
            0xBC => "Phone Key A".to_string(),
            0xBD => "Phone Key B".to_string(),
            0xBE => "Phone Key C".to_string(),
            0xBF => "Phone Key D".to_string(),
            0xC0 => "Phone Call History Key".to_string(),
            0xC1 => "Phone Caller ID Key".to_string(),
            0xC2 => "Settings Key".to_string(),
            0xC3..=0xEF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_consumer_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Consumer Control".to_string(),
            0x02 => "Numeric Key Pad".to_string(),
            0x03 => "Programmable Buttons".to_string(),
            0x04 => "Microphone".to_string(),
            0x05 => "Headphone".to_string(),
            0x06 => "Graphic Equalizer".to_string(),
            0x07..=0x1F => "Reserved".to_string(),
            0x20 => "+10".to_string(),
            0x21 => "+100".to_string(),
            0x22 => "AM/PM".to_string(),
            0x23..=0x2F => "Reserved".to_string(),
            0x30 => "Power".to_string(),
            0x31 => "Reset".to_string(),
            0x32 => "Sleep".to_string(),
            0x33 => "Sleep After".to_string(),
            0x34 => "Sleep Mode".to_string(),
            0x35 => "Illumination".to_string(),
            0x36 => "Function Buttons".to_string(),
            0x37..=0x3F => "Reserved".to_string(),
            0x40 => "Menu".to_string(),
            0x41 => "Menu Pick".to_string(),
            0x42 => "Menu Up".to_string(),
            0x43 => "Menu Down".to_string(),
            0x44 => "Menu Left".to_string(),
            0x45 => "Menu Right".to_string(),
            0x46 => "Menu Escape".to_string(),
            0x47 => "Menu Value Increase".to_string(),
            0x48 => "Menu Value Decrease".to_string(),
            0x49..=0x5F => "Reserved".to_string(),
            0x60 => "Data On Screen".to_string(),
            0x61 => "Closed Caption".to_string(),
            0x62 => "Closed Caption Select".to_string(),
            0x63 => "VCR/TV".to_string(),
            0x64 => "Broadcast Mode".to_string(),
            0x65 => "Snapshot".to_string(),
            0x66 => "Still".to_string(),
            0x67 => "Picture-in-Picture Toggle".to_string(),
            0x68 => "Picture-in-Picture Swap".to_string(),
            0x69 => "Red Menu Button".to_string(),
            0x6A => "Green Menu Button".to_string(),
            0x6B => "Blue Menu Button".to_string(),
            0x6C => "Yellow Menu Button".to_string(),
            0x6D => "Aspect".to_string(),
            0x6E => "3D Mode Select".to_string(),
            0x6F => "Display Brightness Increment".to_string(),
            0x70 => "Display Brightness Decrement".to_string(),
            0x71 => "Display Brightness".to_string(),
            0x72 => "Display Backlight Toggle".to_string(),
            0x73 => "Display Set Brightness to Minimum".to_string(),
            0x74 => "Display Set Brightness to Maximum".to_string(),
            0x75 => "Display Set Auto Brightness".to_string(),
            0x76 => "Camera Access Enabled".to_string(),
            0x77 => "Camera Access Disabled".to_string(),
            0x78 => "Camera Access Toggle".to_string(),
            0x79 => "Keyboard Brightness Increment".to_string(),
            0x7A => "Keyboard Brightness Decrement".to_string(),
            0x7B => "Keyboard Backlight Set Level".to_string(),
            0x7C => "Keyboard Backlight OOC".to_string(),
            0x7D => "Keyboard Backlight Set Minimum".to_string(),
            0x7E => "Keyboard Backlight Set Maximum".to_string(),
            0x7F => "Keyboard Backlight Auto".to_string(),
            0x80 => "Selection".to_string(),
            0x81 => "Assign Selection".to_string(),
            0x82 => "Mode Step".to_string(),
            0x83 => "Recall Last".to_string(),
            0x84 => "Enter Channel".to_string(),
            0x85 => "Order Movie".to_string(),
            0x86 => "Channel".to_string(),
            0x87 => "Media Selection".to_string(),
            0x88 => "Media Select Computer".to_string(),
            0x89 => "Media Select TV".to_string(),
            0x8A => "Media Select WWW".to_string(),
            0x8B => "Media Select DVD".to_string(),
            0x8C => "Media Select Telephone".to_string(),
            0x8D => "Media Select Program Guide".to_string(),
            0x8E => "Media Select Video Phone".to_string(),
            0x8F => "Media Select Games".to_string(),
            0x90 => "Media Select Messages".to_string(),
            0x91 => "Media Select CD".to_string(),
            0x92 => "Media Select VCR".to_string(),
            0x93 => "Media Select Tuner".to_string(),
            0x94 => "Quit".to_string(),
            0x95 => "Help".to_string(),
            0x96 => "Media Select Tape".to_string(),
            0x97 => "Media Select Cable".to_string(),
            0x98 => "Media Select Satellite".to_string(),
            0x99 => "Media Select Security".to_string(),
            0x9A => "Media Select Home".to_string(),
            0x9B => "Media Select Call".to_string(),
            0x9C => "Channel Increment".to_string(),
            0x9D => "Channel Decrement".to_string(),
            0x9E => "Media Select SAP".to_string(),
            0x9F..=0x9F => "Reserved".to_string(),
            0xA0 => "VCR Plus".to_string(),
            0xA1 => "Once".to_string(),
            0xA2 => "Daily".to_string(),
            0xA3 => "Weekly".to_string(),
            0xA4 => "Monthly".to_string(),
            0xA5..=0xAF => "Reserved".to_string(),
            0xB0 => "Play".to_string(),
            0xB1 => "Pause".to_string(),
            0xB2 => "Record".to_string(),
            0xB3 => "Fast Forward".to_string(),
            0xB4 => "Rewind".to_string(),
            0xB5 => "Scan Next Track".to_string(),
            0xB6 => "Scan Previous Track".to_string(),
            0xB7 => "Stop".to_string(),
            0xB8 => "Eject".to_string(),
            0xB9 => "Random Play".to_string(),
            0xBA => "Select Disc".to_string(),
            0xBB => "Enter Disc".to_string(),
            0xBC => "Repeat".to_string(),
            0xBD => "Tracking".to_string(),
            0xBE => "Track Normal".to_string(),
            0xBF => "Slow Tracking".to_string(),
            0xC0 => "Frame Forward".to_string(),
            0xC1 => "Frame Back".to_string(),
            0xC2 => "Mark".to_string(),
            0xC3 => "Clear Mark".to_string(),
            0xC4 => "Repeat From Mark".to_string(),
            0xC5 => "Return To Mark".to_string(),
            0xC6 => "Search Mark Forward".to_string(),
            0xC7 => "Search Mark Backwards".to_string(),
            0xC8 => "Counter Reset".to_string(),
            0xC9 => "Show Counter".to_string(),
            0xCA => "Tracking Increment".to_string(),
            0xCB => "Tracking Decrement".to_string(),
            0xCC => "Stop/Eject".to_string(),
            0xCD => "Play/Pause".to_string(),
            0xCE => "Play/Skip".to_string(),
            0xCF => "Voice Command".to_string(),
            0xD0 => "Invoke Capture Interface".to_string(),
            0xD1 => "Start or Stop Game Recording".to_string(),
            0xD2 => "Historical Game Capture".to_string(),
            0xD3 => "Capture Game Screenshot".to_string(),
            0xD4 => "Show or Hide Recording Indicator".to_string(),
            0xD5 => "Start or Stop Microphone Capture".to_string(),
            0xD6 => "Start or Stop Camera Capture".to_string(),
            0xD7 => "Start or Stop Game Broadcast".to_string(),
            0xD8 => "Start or Stop Voice Dictation Session".to_string(),
            0xD9..=0xDF => "Reserved".to_string(),
            0xE0 => "Volume".to_string(),
            0xE1 => "Balance".to_string(),
            0xE2 => "Mute".to_string(),
            0xE3 => "Bass".to_string(),
            0xE4 => "Treble".to_string(),
            0xE5 => "Bass Boost".to_string(),
            0xE6 => "Surround Mode".to_string(),
            0xE7 => "Loudness".to_string(),
            0xE8 => "MPX".to_string(),
            0xE9 => "Volume Increment".to_string(),
            0xEA => "Volume Decrement".to_string(),
            0xEB..=0xEF => "Reserved".to_string(),
            0xF0 => "Speed Select".to_string(),
            0xF1 => "Playback Speed".to_string(),
            0xF2 => "Standard Play".to_string(),
            0xF3 => "Long Play".to_string(),
            0xF4 => "Extended Play".to_string(),
            0xF5 => "Slow".to_string(),
            0xF6..=0xFF => "Reserved".to_string(),
            0x100 => "Fan Enable".to_string(),
            0x101 => "Fan Speed".to_string(),
            0x102 => "Light Enable".to_string(),
            0x103 => "Light Illumination Level".to_string(),
            0x104 => "Climate Control Enable".to_string(),
            0x105 => "Room Temperature".to_string(),
            0x106 => "Security Enable".to_string(),
            0x107 => "Fire Alarm".to_string(),
            0x108 => "Police Alarm".to_string(),
            0x109 => "Proximity".to_string(),
            0x10A => "Motion".to_string(),
            0x10B => "Duress Alarm".to_string(),
            0x10C => "Holdup Alarm".to_string(),
            0x10D => "Medical Alarm".to_string(),
            0x10E..=0x14F => "Reserved".to_string(),
            0x150 => "Balance Right".to_string(),
            0x151 => "Balance Left".to_string(),
            0x152 => "Bass Increment".to_string(),
            0x153 => "Bass Decrement".to_string(),
            0x154 => "Treble Increment".to_string(),
            0x155 => "Treble Decrement".to_string(),
            0x156..=0x15F => "Reserved".to_string(),
            0x160 => "Speaker System".to_string(),
            0x161 => "Channel Left".to_string(),
            0x162 => "Channel Right".to_string(),
            0x163 => "Channel Center".to_string(),
            0x164 => "Channel Front".to_string(),
            0x165 => "Channel Center Front".to_string(),
            0x166 => "Channel Side".to_string(),
            0x167 => "Channel Surround".to_string(),
            0x168 => "Channel Low Frequency Enhancement".to_string(),
            0x169 => "Channel Top".to_string(),
            0x16A => "Channel Unknown".to_string(),
            0x16B..=0x16F => "Reserved".to_string(),
            0x170 => "Sub-channel".to_string(),
            0x171 => "Sub-channel Increment".to_string(),
            0x172 => "Sub-channel Decrement".to_string(),
            0x173 => "Alternate Audio Increment".to_string(),
            0x174 => "Alternate Audio Decrement".to_string(),
            0x175..=0x17F => "Reserved".to_string(),
            0x180 => "Application Launch Buttons".to_string(),
            0x181 => "AL Launch Button Configuration Tool".to_string(),
            0x182 => "AL Programmable Button Configuration".to_string(),
            0x183 => "AL Consumer Control Configuration".to_string(),
            0x184 => "AL Word Processor".to_string(),
            0x185 => "AL Text Editor".to_string(),
            0x186 => "AL Spreadsheet".to_string(),
            0x187 => "AL Graphics Editor".to_string(),
            0x188 => "AL Presentation App".to_string(),
            0x189 => "AL Database App".to_string(),
            0x18A => "AL Email Reader".to_string(),
            0x18B => "AL Newsreader".to_string(),
            0x18C => "AL Voicemail".to_string(),
            0x18D => "AL Contacts/Address Book".to_string(),
            0x18E => "AL Calendar/Schedule".to_string(),
            0x18F => "AL Task/Project Manager".to_string(),
            0x190 => "AL Log/Journal/Timecard".to_string(),
            0x191 => "AL Checkbook/Finance".to_string(),
            0x192 => "AL Calculator".to_string(),
            0x193 => "AL A/V Capture/Playback".to_string(),
            0x194 => "AL Local Machine Browser".to_string(),
            0x195 => "AL LAN/WAN Browser".to_string(),
            0x196 => "AL Internet Browser".to_string(),
            0x197 => "AL Remote Networking/ISP Connect".to_string(),
            0x198 => "AL Network Conference".to_string(),
            0x199 => "AL Network Chat".to_string(),
            0x19A => "AL Telephony/Dialer".to_string(),
            0x19B => "AL Logon".to_string(),
            0x19C => "AL Logoff".to_string(),
            0x19D => "AL Logon/Logoff".to_string(),
            0x19E => "AL Terminal Lock/Screensaver".to_string(),
            0x19F => "AL Control Panel".to_string(),
            0x1A0 => "AL Command Line Processor/Run".to_string(),
            0x1A1 => "AL Process/Task Manager".to_string(),
            0x1A2 => "AL Select Task/Application".to_string(),
            0x1A3 => "AL Next Task/Application".to_string(),
            0x1A4 => "AL Previous Task/Application".to_string(),
            0x1A5 => "AL Preemptive Halt Task/Application".to_string(),
            0x1A6 => "AL Integrated Help Center".to_string(),
            0x1A7 => "AL Documents".to_string(),
            0x1A8 => "AL Thesaurus".to_string(),
            0x1A9 => "AL Dictionary".to_string(),
            0x1AA => "AL Desktop".to_string(),
            0x1AB => "AL Spell Check".to_string(),
            0x1AC => "AL Grammar Check".to_string(),
            0x1AD => "AL Wireless Status".to_string(),
            0x1AE => "AL Keyboard Layout".to_string(),
            0x1AF => "AL Virus Protection".to_string(),
            0x1B0 => "AL Encryption".to_string(),
            0x1B1 => "AL Screen Saver".to_string(),
            0x1B2 => "AL Alarms".to_string(),
            0x1B3 => "AL Clock".to_string(),
            0x1B4 => "AL File Browser".to_string(),
            0x1B5 => "AL Power Status".to_string(),
            0x1B6 => "AL Image Browser".to_string(),
            0x1B7 => "AL Audio Browser".to_string(),
            0x1B8 => "AL Movie Browser".to_string(),
            0x1B9 => "AL Digital Rights Manager".to_string(),
            0x1BA => "AL Digital Wallet".to_string(),
            0x1BB..=0x1BB => "Reserved".to_string(),
            0x1BC => "AL Instant Messaging".to_string(),
            0x1BD => "AL OEM Features/ Tips/Tutorial Browser".to_string(),
            0x1BE => "AL OEM Help".to_string(),
            0x1BF => "AL Online Community".to_string(),
            0x1C0 => "AL Entertainment Content Browser".to_string(),
            0x1C1 => "AL Online Shopping Browser".to_string(),
            0x1C2 => "AL SmartCard Information/Help".to_string(),
            0x1C3 => "AL Market Monitor/Finance Browser".to_string(),
            0x1C4 => "AL Customized Corporate News Browser".to_string(),
            0x1C5 => "AL Online Activity Browser".to_string(),
            0x1C6 => "AL Research/Search Browser".to_string(),
            0x1C7 => "AL Audio Player".to_string(),
            0x1C8 => "AL Message Status".to_string(),
            0x1C9 => "AL Contact Sync".to_string(),
            0x1CA => "AL Navigation".to_string(),
            0x1CB => "AL Context-aware Desktop Assistant".to_string(),
            0x1CC..=0x1FF => "Reserved".to_string(),
            0x200 => "Generic GUI Application Controls".to_string(),
            0x201 => "AC New".to_string(),
            0x202 => "AC Open".to_string(),
            0x203 => "AC Close".to_string(),
            0x204 => "AC Exit".to_string(),
            0x205 => "AC Maximize".to_string(),
            0x206 => "AC Minimize".to_string(),
            0x207 => "AC Save".to_string(),
            0x208 => "AC Print".to_string(),
            0x209 => "AC Properties".to_string(),
            0x20A..=0x219 => "Reserved".to_string(),
            0x21A => "AC Undo".to_string(),
            0x21B => "AC Copy".to_string(),
            0x21C => "AC Cut".to_string(),
            0x21D => "AC Paste".to_string(),
            0x21E => "AC Select All".to_string(),
            0x21F => "AC Find".to_string(),
            0x220 => "AC Find and Replace".to_string(),
            0x221 => "AC Search".to_string(),
            0x222 => "AC Go To".to_string(),
            0x223 => "AC Home".to_string(),
            0x224 => "AC Back".to_string(),
            0x225 => "AC Forward".to_string(),
            0x226 => "AC Stop".to_string(),
            0x227 => "AC Refresh".to_string(),
            0x228 => "AC Previous Link".to_string(),
            0x229 => "AC Next Link".to_string(),
            0x22A => "AC Bookmarks".to_string(),
            0x22B => "AC History".to_string(),
            0x22C => "AC Subscriptions".to_string(),
            0x22D => "AC Zoom In".to_string(),
            0x22E => "AC Zoom Out".to_string(),
            0x22F => "AC Zoom".to_string(),
            0x230 => "AC Full Screen View".to_string(),
            0x231 => "AC Normal View".to_string(),
            0x232 => "AC View Toggle".to_string(),
            0x233 => "AC Scroll Up".to_string(),
            0x234 => "AC Scroll Down".to_string(),
            0x235 => "AC Scroll".to_string(),
            0x236 => "AC Pan Left".to_string(),
            0x237 => "AC Pan Right".to_string(),
            0x238 => "AC Pan".to_string(),
            0x239 => "AC New Window".to_string(),
            0x23A => "AC Tile Horizontally".to_string(),
            0x23B => "AC Tile Vertically".to_string(),
            0x23C => "AC Format".to_string(),
            0x23D => "AC Edit".to_string(),
            0x23E => "AC Bold".to_string(),
            0x23F => "AC Italics".to_string(),
            0x240 => "AC Underline".to_string(),
            0x241 => "AC Strikethrough".to_string(),
            0x242 => "AC Subscript".to_string(),
            0x243 => "AC Superscript".to_string(),
            0x244 => "AC All Caps".to_string(),
            0x245 => "AC Rotate".to_string(),
            0x246 => "AC Resize".to_string(),
            0x247 => "AC Flip Horizontal".to_string(),
            0x248 => "AC Flip Vertical".to_string(),
            0x249 => "AC Mirror Horizontal".to_string(),
            0x24A => "AC Mirror Vertical".to_string(),
            0x24B => "AC Font Select".to_string(),
            0x24C => "AC Font Color".to_string(),
            0x24D => "AC Font Size".to_string(),
            0x24E => "AC Justify Left".to_string(),
            0x24F => "AC Justify Center H".to_string(),
            0x250 => "AC Justify Right".to_string(),
            0x251 => "AC Justify Block H".to_string(),
            0x252 => "AC Justify Top".to_string(),
            0x253 => "AC Justify Center V".to_string(),
            0x254 => "AC Justify Bottom".to_string(),
            0x255 => "AC Justify Block V".to_string(),
            0x256 => "AC Indent Decrease".to_string(),
            0x257 => "AC Indent Increase".to_string(),
            0x258 => "AC Numbered List".to_string(),
            0x259 => "AC Restart Numbering".to_string(),
            0x25A => "AC Bulleted List".to_string(),
            0x25B => "AC Promote".to_string(),
            0x25C => "AC Demote".to_string(),
            0x25D => "AC Yes".to_string(),
            0x25E => "AC No".to_string(),
            0x25F => "AC Cancel".to_string(),
            0x260 => "AC Catalog".to_string(),
            0x261 => "AC Buy/Checkout".to_string(),
            0x262 => "AC Add to Cart".to_string(),
            0x263 => "AC Expand".to_string(),
            0x264 => "AC Expand All".to_string(),
            0x265 => "AC Collapse".to_string(),
            0x266 => "AC Collapse All".to_string(),
            0x267 => "AC Print Preview".to_string(),
            0x268 => "AC Paste Special".to_string(),
            0x269 => "AC Insert Mode".to_string(),
            0x26A => "AC Delete".to_string(),
            0x26B => "AC Lock".to_string(),
            0x26C => "AC Unlock".to_string(),
            0x26D => "AC Protect".to_string(),
            0x26E => "AC Unprotect".to_string(),
            0x26F => "AC Attach Comment".to_string(),
            0x270 => "AC Delete Comment".to_string(),
            0x271 => "AC View Comment".to_string(),
            0x272 => "AC Select Word".to_string(),
            0x273 => "AC Select Sentence".to_string(),
            0x274 => "AC Select Paragraph".to_string(),
            0x275 => "AC Select Column".to_string(),
            0x276 => "AC Select Row".to_string(),
            0x277 => "AC Select Table".to_string(),
            0x278 => "AC Select Object".to_string(),
            0x279 => "AC Redo/Repeat".to_string(),
            0x27A => "AC Sort".to_string(),
            0x27B => "AC Sort Ascending".to_string(),
            0x27C => "AC Sort Descending".to_string(),
            0x27D => "AC Filter".to_string(),
            0x27E => "AC Set Clock".to_string(),
            0x27F => "AC View Clock".to_string(),
            0x280 => "AC Select Time Zone".to_string(),
            0x281 => "AC Edit Time Zones".to_string(),
            0x282 => "AC Set Alarm".to_string(),
            0x283 => "AC Clear Alarm".to_string(),
            0x284 => "AC Snooze Alarm".to_string(),
            0x285 => "AC Reset Alarm".to_string(),
            0x286 => "AC Synchronize".to_string(),
            0x287 => "AC Send/Receive".to_string(),
            0x288 => "AC Send To".to_string(),
            0x289 => "AC Reply".to_string(),
            0x28A => "AC Reply All".to_string(),
            0x28B => "AC Forward Msg".to_string(),
            0x28C => "AC Send".to_string(),
            0x28D => "AC Attach File".to_string(),
            0x28E => "AC Upload".to_string(),
            0x28F => "AC Download (Save Target As)".to_string(),
            0x290 => "AC Set Borders".to_string(),
            0x291 => "AC Insert Row".to_string(),
            0x292 => "AC Insert Column".to_string(),
            0x293 => "AC Insert File".to_string(),
            0x294 => "AC Insert Picture".to_string(),
            0x295 => "AC Insert Object".to_string(),
            0x296 => "AC Insert Symbol".to_string(),
            0x297 => "AC Save and Close".to_string(),
            0x298 => "AC Rename".to_string(),
            0x299 => "AC Merge".to_string(),
            0x29A => "AC Split".to_string(),
            0x29B => "AC Disribute Horizontally".to_string(),
            0x29C => "AC Distribute Vertically".to_string(),
            0x29D => "AC Next Keyboard Layout Select".to_string(),
            0x29E => "AC Navigation Guidance".to_string(),
            0x29F => "AC Desktop Show All Windows".to_string(),
            0x2A0 => "AC Soft Key Left".to_string(),
            0x2A1 => "AC Soft Key Right".to_string(),
            0x2A2 => "AC Desktop Show All Applications".to_string(),
            0x2A3..=0x2AF => "Reserved".to_string(),
            0x2B0 => "AC Idle Keep Alive".to_string(),
            0x2B1..=0x2BF => "Reserved".to_string(),
            0x2C0 => "Extended Keyboard Attributes Collection".to_string(),
            0x2C1 => "Keyboard Form Factor".to_string(),
            0x2C2 => "Keyboard Key Type".to_string(),
            0x2C3 => "Keyboard Physical Layout".to_string(),
            0x2C4 => "Vendor-Specific Keyboard Physical Layout".to_string(),
            0x2C5 => "Keyboard IETF Language Tag Index".to_string(),
            0x2C6 => "Implemented Keyboard Input Assist Controls".to_string(),
            0x2C7 => "Keyboard Input Assist Previous".to_string(),
            0x2C8 => "Keyboard Input Assist Next".to_string(),
            0x2C9 => "Keyboard Input Assist Previous Group".to_string(),
            0x2CA => "Keyboard Input Assist Next Group".to_string(),
            0x2CB => "Keyboard Input Assist Accept".to_string(),
            0x2CC => "Keyboard Input Assist Cancel".to_string(),
            0x2CD..=0x2CF => "Reserved".to_string(),
            0x2D0 => "Privacy Screen Toggle".to_string(),
            0x2D1 => "Privacy Screen Level Decrement".to_string(),
            0x2D2 => "Privacy Screen Level Increment".to_string(),
            0x2D3 => "Privacy Screen Level Minimum".to_string(),
            0x2D4 => "Privacy Screen Level Maximum".to_string(),
            0x2D5..=0x4FF => "Reserved".to_string(),
            0x500 => "Contact Edited".to_string(),
            0x501 => "Contact Added".to_string(),
            0x502 => "Contact Record Active".to_string(),
            0x503 => "Contact Index".to_string(),
            0x504 => "Contact Nickname".to_string(),
            0x505 => "Contact First Name".to_string(),
            0x506 => "Contact Last Name".to_string(),
            0x507 => "Contact Full Name".to_string(),
            0x508 => "Contact Phone Number Personal".to_string(),
            0x509 => "Contact Phone Number Business".to_string(),
            0x50A => "Contact Phone Number Mobile".to_string(),
            0x50B => "Contact Phone Number Pager".to_string(),
            0x50C => "Contact Phone Number Fax".to_string(),
            0x50D => "Contact Phone Number Other".to_string(),
            0x50E => "Contact Email Personal".to_string(),
            0x50F => "Contact Email Business".to_string(),
            0x510 => "Contact Email Other".to_string(),
            0x511 => "Contact Email Main".to_string(),
            0x512 => "Contact Speed Dial Number".to_string(),
            0x513 => "Contact Status Flag".to_string(),
            0x514 => "Contact Misc.".to_string(),
            0x515..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_digitizers_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Digitizer".to_string(),
            0x02 => "Pen".to_string(),
            0x03 => "Light Pen".to_string(),
            0x04 => "Touch Screen".to_string(),
            0x05 => "Touch Pad".to_string(),
            0x06 => "Whiteboard".to_string(),
            0x07 => "Coordinate Measuring Machine".to_string(),
            0x08 => "3D Digitizer".to_string(),
            0x09 => "Stereo Plotter".to_string(),
            0x0A => "Articulated Arm".to_string(),
            0x0B => "Armature".to_string(),
            0x0C => "Multiple Point Digitizer".to_string(),
            0x0D => "Free Space Wand".to_string(),
            0x0E => "Device Configuration".to_string(),
            0x0F => "Capacitive Heat Map Digitizer".to_string(),
            0x10..=0x1F => "Reserved".to_string(),
            0x20 => "Stylus [55] CA/CL 16.2".to_string(),
            0x21 => "Puck".to_string(),
            0x22 => "Finger".to_string(),
            0x23 => "Device settings".to_string(),
            0x24 => "Character Gesture".to_string(),
            0x25..=0x2F => "Reserved".to_string(),
            0x30 => "Tip Pressure".to_string(),
            0x31 => "Barrel Pressure".to_string(),
            0x32 => "In Range".to_string(),
            0x33 => "Touch".to_string(),
            0x34 => "Untouch".to_string(),
            0x35 => "Tap".to_string(),
            0x36 => "Quality".to_string(),
            0x37 => "Data Valid".to_string(),
            0x38 => "Transducer Index".to_string(),
            0x39 => "Tablet Function Keys".to_string(),
            0x3A => "Program Change Keys".to_string(),
            0x3B => "Battery Strength".to_string(),
            0x3C => "Invert".to_string(),
            0x3D => "X Tilt".to_string(),
            0x3E => "Y Tilt".to_string(),
            0x3F => "Azimuth".to_string(),
            0x40 => "Altitude".to_string(),
            0x41 => "Twist".to_string(),
            0x42 => "Tip Switch".to_string(),
            0x43 => "Secondary Tip Switch".to_string(),
            0x44 => "Barrel Switch".to_string(),
            0x45 => "Eraser".to_string(),
            0x46 => "Tablet Pick".to_string(),
            0x47 => "Touch Valid".to_string(),
            0x48 => "Width".to_string(),
            0x49 => "Height".to_string(),
            0x4A..=0x50 => "Reserved".to_string(),
            0x51 => "Contact Identifier".to_string(),
            0x52 => "Device Mode".to_string(),
            0x53 => "Device Identifier [7] DV/SV 16.7".to_string(),
            0x54 => "Contact Count".to_string(),
            0x55 => "Contact Count Maximum".to_string(),
            0x56 => "Scan Time".to_string(),
            0x57 => "Surface Switch".to_string(),
            0x58 => "Button Switch".to_string(),
            0x59 => "Pad Type".to_string(),
            0x5A => "Secondary Barrel Switch".to_string(),
            0x5B => "Transducer Serial Number".to_string(),
            0x5C => "Preferred Color".to_string(),
            0x5D => "Preferred Color is Locked".to_string(),
            0x5E => "Preferred Line Width".to_string(),
            0x5F => "Preferred Line Width is Locked".to_string(),
            0x60 => "Latency Mode".to_string(),
            0x61 => "Gesture Character Quality".to_string(),
            0x62 => "Character Gesture Data Length".to_string(),
            0x63 => "Character Gesture Data".to_string(),
            0x64 => "Gesture Character Encoding".to_string(),
            0x65 => "UTF8 Character Gesture Encoding".to_string(),
            0x66 => "UTF16 Little Endian Character Gesture Encoding".to_string(),
            0x67 => "UTF16 Big Endian Character Gesture Encoding".to_string(),
            0x68 => "UTF32 Little Endian Character Gesture Encoding".to_string(),
            0x69 => "UTF32 Big Endian Character Gesture Encoding".to_string(),
            0x6A => "Capacitive Heat Map Protocol Vendor ID".to_string(),
            0x6B => "Capacitive Heat Map Protocol Version".to_string(),
            0x6C => "Capacitive Heat Map Frame Data".to_string(),
            0x6D => "Gesture Character Enable".to_string(),
            0x6E..=0x6F => "Reserved".to_string(),
            0x70 => "Preferred Line Style".to_string(),
            0x71 => "Preferred Line Style is Locked".to_string(),
            0x72 => "Ink".to_string(),
            0x73 => "Pencil".to_string(),
            0x74 => "Highlighter".to_string(),
            0x75 => "Chisel Marker".to_string(),
            0x76 => "Brush".to_string(),
            0x77 => "No Preference".to_string(),
            0x78..=0x7F => "Reserved".to_string(),
            0x80 => "Digitizer Diagnostic".to_string(),
            0x81 => "Digitizer Error".to_string(),
            0x82 => "Err Normal Status".to_string(),
            0x83 => "Err Transducers Exceeded".to_string(),
            0x84 => "Err Full Trans Features Unavailable".to_string(),
            0x85 => "Err Charge Low".to_string(),
            0x86..=0x8F => "Reserved".to_string(),
            0x90 => "Transducer Software Info".to_string(),
            0x91 => "Transducer Vendor Id".to_string(),
            0x92 => "Transducer Product Id".to_string(),
            0x93 => "Device Supported Protocols".to_string(),
            0x94 => "Transducer Supported Protocols".to_string(),
            0x95 => "No Protocol".to_string(),
            0x96 => "Wacom AES Protocol".to_string(),
            0x97 => "USI Protocol".to_string(),
            0x98 => "Microsoft Pen Protocol".to_string(),
            0x99..=0x9F => "Reserved".to_string(),
            0xA0 => "Supported Report Rates".to_string(),
            0xA1 => "Report Rate".to_string(),
            0xA2 => "Transducer Connected".to_string(),
            0xA3 => "Switch Disabled".to_string(),
            0xA4 => "Switch Unimplemented".to_string(),
            0xA5 => "Transducer Switches".to_string(),
            0xA6..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_haptics_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Simple Haptic Controller".to_string(),
            0x02..=0x0F => "Reserved".to_string(),
            0x10 => "Waveform List".to_string(),
            0x11 => "Duration List".to_string(),
            0x12..=0x1F => "Reserved".to_string(),
            0x20 => "Auto Trigger".to_string(),
            0x21 => "Manual Trigger".to_string(),
            0x22 => "Auto Trigger Associated Control".to_string(),
            0x23 => "Intensity".to_string(),
            0x24 => "Repeat Count".to_string(),
            0x25 => "Retrigger Period".to_string(),
            0x26 => "Waveform Vendor Page".to_string(),
            0x27 => "Waveform Vendor ID".to_string(),
            0x28 => "Waveform Cutoff Time".to_string(),
            0x29..=0x1000 => "Reserved".to_string(),
            0x1001 => "Waveform None".to_string(),
            0x1002 => "Waveform Stop".to_string(),
            0x1003 => "Waveform Click".to_string(),
            0x1004 => "Waveform Buzz Continuous".to_string(),
            0x1005 => "Waveform Rumble Continuous".to_string(),
            0x1006 => "Waveform Press".to_string(),
            0x1007 => "Waveform Release".to_string(),
            0x1008..=0x2000 => "Reserved".to_string(),
            0x2001..=0x2FFF => "Reserved for Vendor Waveforms".to_string(),
            0x3000..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_physical_interface_device_usage_name(usage: u32) -> String {
        match usage {
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_unicode_usage_name(usage: u32) -> String {
        match usage {
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_eye_and_head_trackers_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Eye Tracker".to_string(),
            0x02 => "Head Tracker".to_string(),
            0x03..=0x0F => "Reserved".to_string(),
            0x10 => "Tracking Data".to_string(),
            0x11 => "Capabilities".to_string(),
            0x12 => "Configuration".to_string(),
            0x13 => "Status".to_string(),
            0x14 => "Control".to_string(),
            0x15..=0x1F => "Reserved".to_string(),
            0x20 => "Sensor Timestamp".to_string(),
            0x21 => "Position X".to_string(),
            0x22 => "Position Y".to_string(),
            0x23 => "Position Z".to_string(),
            0x24 => "Gaze Point".to_string(),
            0x25 => "Left Eye Position".to_string(),
            0x26 => "Right Eye Position".to_string(),
            0x27 => "Head Position".to_string(),
            0x28 => "Head Direction Point".to_string(),
            0x29 => "Rotation about X axis".to_string(),
            0x2A => "Rotation about Y axis".to_string(),
            0x2B => "Rotation about Z axis".to_string(),
            0x2C..=0xFF => "Reserved".to_string(),
            0x100 => "Tracker Quality".to_string(),
            0x101 => "Minimum Tracking Distance".to_string(),
            0x102 => "Optimum Tracking Distance".to_string(),
            0x103 => "Maximum Tracking Distance".to_string(),
            0x104 => "Maximum Screen Plane Width".to_string(),
            0x105 => "Maximum Screen Plane Height".to_string(),
            0x106..=0x1FF => "Reserved".to_string(),
            0x200 => "Display Manufacturer ID".to_string(),
            0x201 => "Display Product ID".to_string(),
            0x202 => "Display Serial Number".to_string(),
            0x203 => "Display Manufacturer Date".to_string(),
            0x204 => "Calibrated Screen Width".to_string(),
            0x205 => "Calibrated Screen Height".to_string(),
            0x206..=0x2FF => "Reserved".to_string(),
            0x300 => "Sampling Frequency".to_string(),
            0x301 => "Configuration Status".to_string(),
            0x302..=0x3FF => "Reserved".to_string(),
            0x400 => "Device Mode Request".to_string(),
            0x401..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_auxiliary_display_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Alphanumeric Display".to_string(),
            0x02 => "Auxiliary Display".to_string(),
            0x03..=0x1F => "Reserved".to_string(),
            0x20 => "Display Attributes Report".to_string(),
            0x21 => "ASCII Character Set".to_string(),
            0x22 => "Data Read Back".to_string(),
            0x23 => "Font Read Back".to_string(),
            0x24 => "Display Control Report".to_string(),
            0x25 => "Clear Display".to_string(),
            0x26 => "Display Enable".to_string(),
            0x27 => "Screen Saver Delay SV/DV 20.1.2".to_string(),
            0x28 => "Screen Saver Enable".to_string(),
            0x29 => "Vertical Scroll SF/DF 20.1.3".to_string(),
            0x2A => "Horizontal Scroll SF/DF 20.1.3".to_string(),
            0x2B => "Character Report".to_string(),
            0x2C => "Display Data".to_string(),
            0x2D => "Display Status".to_string(),
            0x2E => "Stat Not Ready".to_string(),
            0x2F => "Stat Ready".to_string(),
            0x30 => "Err Not a loadable character".to_string(),
            0x31 => "Err Font data cannot be read".to_string(),
            0x32 => "Cursor Position Report".to_string(),
            0x33 => "Row".to_string(),
            0x34 => "Column".to_string(),
            0x35 => "Rows".to_string(),
            0x36 => "Columns".to_string(),
            0x37 => "Cursor Pixel Positioning".to_string(),
            0x38 => "Cursor Mode".to_string(),
            0x39 => "Cursor Enable".to_string(),
            0x3A => "Cursor Blink".to_string(),
            0x3B => "Font Report".to_string(),
            0x3C => "Font Data Buffered Bytes 20.1.7".to_string(),
            0x3D => "Character Width".to_string(),
            0x3E => "Character Height".to_string(),
            0x3F => "Character Spacing Horizontal".to_string(),
            0x40 => "Character Spacing Vertical".to_string(),
            0x41 => "Unicode Character Set".to_string(),
            0x42 => "Font 7-Segment".to_string(),
            0x43 => "7-Segment Direct Map".to_string(),
            0x44 => "Font 14-Segment".to_string(),
            0x45 => "14-Segment Direct Map".to_string(),
            0x46 => "Display Brightness".to_string(),
            0x47 => "Display Contrast".to_string(),
            0x48 => "Character Attribute".to_string(),
            0x49 => "Attribute Readback".to_string(),
            0x4A => "Attribute Data".to_string(),
            0x4B => "Char Attr Enhance".to_string(),
            0x4C => "Char Attr Underline".to_string(),
            0x4D => "Char Attr Blink".to_string(),
            0x4E..=0x7F => "Reserved".to_string(),
            0x80 => "Bitmap Size X".to_string(),
            0x81 => "Bitmap Size Y".to_string(),
            0x82 => "Max Blit Size".to_string(),
            0x83 => "Bit Depth Format".to_string(),
            0x84 => "Display Orientation".to_string(),
            0x85 => "Palette Report".to_string(),
            0x86 => "Palette Data Size".to_string(),
            0x87 => "Palette Data Offset".to_string(),
            0x88 => "Palette Data [2] Buffered Bytes 20.2.3".to_string(),
            0x89..=0x89 => "Reserved".to_string(),
            0x8A => "Blit Report".to_string(),
            0x8B => "Blit Rectangle X1".to_string(),
            0x8C => "Blit Rectangle Y1".to_string(),
            0x8D => "Blit Rectangle X2".to_string(),
            0x8E => "Blit Rectangle Y2".to_string(),
            0x8F => "Blit Data [2] Buffered Bytes 20.2.4".to_string(),
            0x90 => "Soft Button [2] CL 20.2.1.5".to_string(),
            0x91 => "Soft Button ID [2] SV 20.2.1.5".to_string(),
            0x92 => "Soft Button Side [2] SV 20.2.1.5".to_string(),
            0x93 => "Soft Button Offset 1 [2] SV 20.2.1.5".to_string(),
            0x94 => "Soft Button Offset 2 [2] SV 20.2.1.5".to_string(),
            0x95 => "Soft Button Report".to_string(),
            0x96..=0xC1 => "Reserved".to_string(),
            0xC2 => "Soft Keys".to_string(),
            0xC3..=0xCB => "Reserved".to_string(),
            0xCC => "Display Data Extensions".to_string(),
            0xCD..=0xCE => "Reserved".to_string(),
            0xCF => "Character Mapping".to_string(),
            0xD0..=0xDC => "Reserved".to_string(),
            0xDD => "Unicode Equivalent".to_string(),
            0xDE..=0xDE => "Reserved".to_string(),
            0xDF => "Character Page Mapping".to_string(),
            0xE0..=0xFE => "Reserved".to_string(),
            0xFF => "Request Report".to_string(),
            0x100..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_sensors_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Sensor".to_string(),
            0x02..=0x0F => "Reserved".to_string(),
            0x10 => "Biometric".to_string(),
            0x11 => "Biometric: Human Presence".to_string(),
            0x12 => "Biometric: Human Proximity".to_string(),
            0x13 => "Biometric: Human Touch".to_string(),
            0x14 => "Biometric: Blood Pressure".to_string(),
            0x15 => "Biometric: Body Temperature".to_string(),
            0x16 => "Biometric: Heart Rate".to_string(),
            0x17 => "Biometric: Heart Rate Variability".to_string(),
            0x18 => "Biometric: Peripheral Oxygen Saturation".to_string(),
            0x19 => "Biometric: Respiratory Rate".to_string(),
            0x1A..=0x1F => "Reserved".to_string(),
            0x20 => "Electrical".to_string(),
            0x21 => "Electrical: Capacitance".to_string(),
            0x22 => "Electrical: Current".to_string(),
            0x23 => "Electrical: Power".to_string(),
            0x24 => "Electrical: Inductance".to_string(),
            0x25 => "Electrical: Resistance".to_string(),
            0x26 => "Electrical: Voltage".to_string(),
            0x27 => "Electrical: Potentiometer".to_string(),
            0x28 => "Electrical: Frequency".to_string(),
            0x29 => "Electrical: Period".to_string(),
            0x2A..=0x2F => "Reserved".to_string(),
            0x30 => "Environmental".to_string(),
            0x31 => "Environmental: Atmospheric Pressure".to_string(),
            0x32 => "Environmental: Humidity".to_string(),
            0x33 => "Environmental: Temperature".to_string(),
            0x34 => "Environmental: Wind Direction".to_string(),
            0x35 => "Environmental: Wind Speed".to_string(),
            0x36 => "Environmental: Air Quality".to_string(),
            0x37 => "Environmental: Heat Index".to_string(),
            0x38 => "Environmental: Surface Temperature".to_string(),
            0x39 => "Environmental: Volatile Organic Compounds".to_string(),
            0x3A => "Environmental: Object Presence".to_string(),
            0x3B => "Environmental: Object Proximity".to_string(),
            0x3C..=0x3F => "Reserved".to_string(),
            0x40 => "Light".to_string(),
            0x41 => "Light: Ambient Light".to_string(),
            0x42 => "Light: Consumer Infrared".to_string(),
            0x43 => "Light: Infrared Light".to_string(),
            0x44 => "Light: Visible Light".to_string(),
            0x45 => "Light: Ultraviolet Light".to_string(),
            0x46..=0x4F => "Reserved".to_string(),
            0x50 => "Location".to_string(),
            0x51 => "Location: Broadcast".to_string(),
            0x52 => "Location: Dead Reckoning".to_string(),
            0x53 => "Location: GPS (Global Positioning System)".to_string(),
            0x54 => "Location: Lookup".to_string(),
            0x55 => "Location: Other".to_string(),
            0x56 => "Location: Static".to_string(),
            0x57 => "Location: Triangulation".to_string(),
            0x58..=0x5F => "Reserved".to_string(),
            0x60 => "Mechanical".to_string(),
            0x61 => "Mechanical: Boolean Switch".to_string(),
            0x62 => "Mechanical: Boolean Switch Array".to_string(),
            0x63 => "Mechanical: Multivalue Switch".to_string(),
            0x64 => "Mechanical: Force".to_string(),
            0x65 => "Mechanical: Pressure".to_string(),
            0x66 => "Mechanical: Strain".to_string(),
            0x67 => "Mechanical: Weight".to_string(),
            0x68 => "Mechanical: Haptic Vibrator".to_string(),
            0x69 => "Mechanical: Hall Effect Switch".to_string(),
            0x6A..=0x6F => "Reserved".to_string(),
            0x70 => "Motion".to_string(),
            0x71 => "Motion: Accelerometer 1D".to_string(),
            0x72 => "Motion: Accelerometer 2D".to_string(),
            0x73 => "Motion: Accelerometer 3D".to_string(),
            0x74 => "Motion: Gyrometer 1D".to_string(),
            0x75 => "Motion: Gyrometer 2D".to_string(),
            0x76 => "Motion: Gyrometer 3D".to_string(),
            0x77 => "Motion: Motion Detector".to_string(),
            0x78 => "Motion: Speedometer".to_string(),
            0x79 => "Motion: Accelerometer".to_string(),
            0x7A => "Motion: Gyrometer".to_string(),
            0x7B => "Motion: Gravity Vector".to_string(),
            0x7C => "Motion: Linear Accelerometer".to_string(),
            0x7D..=0x7F => "Reserved".to_string(),
            0x80 => "Orientation".to_string(),
            0x81 => "Orientation: Compass 1D".to_string(),
            0x82 => "Orientation: Compass 2D".to_string(),
            0x83 => "Orientation: Compass 3D".to_string(),
            0x84 => "Orientation: Inclinometer 1D".to_string(),
            0x85 => "Orientation: Inclinometer 2D".to_string(),
            0x86 => "Orientation: Inclinometer 3D".to_string(),
            0x87 => "Orientation: Distance 1D".to_string(),
            0x88 => "Orientation: Distance 2D".to_string(),
            0x89 => "Orientation: Distance 3D".to_string(),
            0x8A => "Orientation: Device Orientation".to_string(),
            0x8B => "Orientation: Compass".to_string(),
            0x8C => "Orientation: Inclinometer".to_string(),
            0x8D => "Orientation: Distance".to_string(),
            0x8E => "Orientation: Relative Orientation".to_string(),
            0x8F => "Orientation: Simple Orientation".to_string(),
            0x90 => "Scanner".to_string(),
            0x91 => "Scanner: Barcode".to_string(),
            0x92 => "Scanner: RFID".to_string(),
            0x93 => "Scanner: NFC".to_string(),
            0x94..=0x9F => "Reserved".to_string(),
            0xA0 => "Time".to_string(),
            0xA1 => "Time: Alarm Timer".to_string(),
            0xA2 => "Time: Real Time Clock".to_string(),
            0xA3..=0xAF => "Reserved".to_string(),
            0xB0 => "Personal Activity".to_string(),
            0xB1 => "Personal Activity: Activity Detection".to_string(),
            0xB2 => "Personal Activity: Device Position".to_string(),
            0xB3 => "Personal Activity: Pedometer".to_string(),
            0xB4 => "Personal Activity: Step Detection".to_string(),
            0xB5..=0xBF => "Reserved".to_string(),
            0xC0 => "Orientation Extended".to_string(),
            0xC1 => "Orientation Extended: Geomagnetic Orientation".to_string(),
            0xC2 => "Orientation Extended: Magnetometer".to_string(),
            0xC3..=0xCF => "Reserved".to_string(),
            0xD0 => "Gesture".to_string(),
            0xD1 => "Gesture: Chassis Flip Gesture".to_string(),
            0xD2 => "Gesture: Hinge Fold Gesture".to_string(),
            0xD3..=0xDF => "Reserved".to_string(),
            0xE0 => "Other".to_string(),
            0xE1 => "Other: Custom".to_string(),
            0xE2 => "Other: Generic".to_string(),
            0xE3 => "Other: Generic Enumerator".to_string(),
            0xE4 => "Other: Hinge Angle".to_string(),
            0xE5..=0xEF => "Reserved".to_string(),
            0xF0 => "Vendor Reserved 1".to_string(),
            0xF1 => "Vendor Reserved 2".to_string(),
            0xF2 => "Vendor Reserved 3".to_string(),
            0xF3 => "Vendor Reserved 4".to_string(),
            0xF4 => "Vendor Reserved 5".to_string(),
            0xF5 => "Vendor Reserved 6".to_string(),
            0xF6 => "Vendor Reserved 7".to_string(),
            0xF7 => "Vendor Reserved 8".to_string(),
            0xF8 => "Vendor Reserved 9".to_string(),
            0xF9 => "Vendor Reserved 10".to_string(),
            0xFA => "Vendor Reserved 11".to_string(),
            0xFB => "Vendor Reserved 12".to_string(),
            0xFC => "Vendor Reserved 13".to_string(),
            0xFD => "Vendor Reserved 14".to_string(),
            0xFE => "Vendor Reserved 15".to_string(),
            0xFF => "Vendor Reserved 16".to_string(),
            0x100..=0x1FF => "Reserved".to_string(),
            0x200 => "Event".to_string(),
            0x201 => "Event: Sensor State".to_string(),
            0x202 => "Event: Sensor Event".to_string(),
            0x203..=0x2FF => "Reserved".to_string(),
            0x300 => "Property".to_string(),
            0x301 => "Property: Friendly Name".to_string(),
            0x302 => "Property: Persistent Unique ID".to_string(),
            0x303 => "Property: Sensor Status".to_string(),
            0x304 => "Property: Minimum Report Interval".to_string(),
            0x305 => "Property: Sensor Manufacturer".to_string(),
            0x306 => "Property: Sensor Model".to_string(),
            0x307 => "Property: Sensor Serial Number".to_string(),
            0x308 => "Property: Sensor Description".to_string(),
            0x309 => "Property: Sensor Connection Type".to_string(),
            0x30A => "Property: Sensor Device Path".to_string(),
            0x30B => "Property: Hardware Revision".to_string(),
            0x30C => "Property: Firmware Version".to_string(),
            0x30D => "Property: Release Date".to_string(),
            0x30E => "Property: Report Interval".to_string(),
            0x30F => "Property: Change Sensitivity Absolute".to_string(),
            0x310 => "Property: Change Sensitivity Percent of Range".to_string(),
            0x311 => "Property: Change Sensitivity Percent Relative".to_string(),
            0x312 => "Property: Accuracy".to_string(),
            0x313 => "Property: Resolution".to_string(),
            0x314 => "Property: Maximum".to_string(),
            0x315 => "Property: Minimum".to_string(),
            0x316 => "Property: Reporting State".to_string(),
            0x317 => "Property: Sampling Rate".to_string(),
            0x318 => "Property: Response Curve".to_string(),
            0x319 => "Property: Power State".to_string(),
            0x31A => "Property: Maximum FIFO Events".to_string(),
            0x31B => "Property: Report Latency".to_string(),
            0x31C => "Property: Flush FIFO Events".to_string(),
            0x31D => "Property: Maximum Power Consumption".to_string(),
            0x31E => "Property: Is Primary".to_string(),
            0x31F..=0x3FF => "Reserved".to_string(),
            0x400 => "Data Field: Location".to_string(),
            0x401..=0x401 => "Reserved".to_string(),
            0x402 => "Data Field: Altitude Antenna Sea Level".to_string(),
            0x403 => "Data Field: Differential Reference Station ID".to_string(),
            0x404 => "Data Field: Altitude Ellipsoid Error".to_string(),
            0x405 => "Data Field: Altitude Ellipsoid".to_string(),
            0x406 => "Data Field: Altitude Sea Level Error".to_string(),
            0x407 => "Data Field: Altitude Sea Level".to_string(),
            0x408 => "Data Field: Differential GPS Data Age".to_string(),
            0x409 => "Data Field: Error Radius".to_string(),
            0x40A => "Data Field: Fix Quality".to_string(),
            0x40B => "Data Field: Fix Type".to_string(),
            0x40C => "Data Field: Geoidal Separation".to_string(),
            0x40D => "Data Field: GPS Operation Mode".to_string(),
            0x40E => "Data Field: GPS Selection Mode".to_string(),
            0x40F => "Data Field: GPS Status".to_string(),
            0x410 => "Data Field: Position Dilution of Precision".to_string(),
            0x411 => "Data Field: Horizontal Dilution of Precision".to_string(),
            0x412 => "Data Field: Vertical Dilution of Precision".to_string(),
            0x413 => "Data Field: Latitude".to_string(),
            0x414 => "Data Field: Longitude".to_string(),
            0x415 => "Data Field: True Heading".to_string(),
            0x416 => "Data Field: Magnetic Heading".to_string(),
            0x417 => "Data Field: Magnetic Variation".to_string(),
            0x418 => "Data Field: Speed".to_string(),
            0x419 => "Data Field: Satellites in View".to_string(),
            0x41A => "Data Field: Satellites in View Azimuth".to_string(),
            0x41B => "Data Field: Satellites in View Elevation".to_string(),
            0x41C => "Data Field: Satellites in View IDs".to_string(),
            0x41D => "Data Field: Satellites in View PRNs".to_string(),
            0x41E => "Data Field: Satellites in View S/N Ratios".to_string(),
            0x41F => "Data Field: Satellites Used Count".to_string(),
            0x420 => "Data Field: Satellites Used PRNs".to_string(),
            0x421 => "Data Field: NMEA Sentence".to_string(),
            0x422 => "Data Field: Address Line 1".to_string(),
            0x423 => "Data Field: Address Line 2".to_string(),
            0x424 => "Data Field: City".to_string(),
            0x425 => "Data Field: State or Province".to_string(),
            0x426 => "Data Field: Country or Region".to_string(),
            0x427 => "Data Field: Postal Code".to_string(),
            0x428..=0x429 => "Reserved".to_string(),
            0x42A => "Property: Location".to_string(),
            0x42B => "Property: Location Desired Accuracy".to_string(),
            0x42C..=0x42F => "Reserved".to_string(),
            0x430 => "Data Field: Environmental".to_string(),
            0x431 => "Data Field: Atmospheric Pressure".to_string(),
            0x432..=0x432 => "Reserved".to_string(),
            0x433 => "Data Field: Relative Humidity".to_string(),
            0x434 => "Data Field: Temperature".to_string(),
            0x435 => "Data Field: Wind Direction".to_string(),
            0x436 => "Data Field: Wind Speed".to_string(),
            0x437 => "Data Field: Air Quality Index".to_string(),
            0x438 => "Data Field: Equivalent CO2".to_string(),
            0x439 => "Data Field: Volatile Organic Compound Concentration".to_string(),
            0x43A => "Data Field: Object Presence".to_string(),
            0x43B => "Data Field: Object Proximity Range".to_string(),
            0x43C => "Data Field: Object Proximity Out of Range".to_string(),
            0x43D..=0x43F => "Reserved".to_string(),
            0x440 => "Property: Environmental".to_string(),
            0x441 => "Property: Reference Pressure".to_string(),
            0x442..=0x44F => "Reserved".to_string(),
            0x450 => "Data Field: Motion".to_string(),
            0x451 => "Data Field: Motion State".to_string(),
            0x452 => "Data Field: Acceleration".to_string(),
            0x453 => "Data Field: Acceleration Axis X".to_string(),
            0x454 => "Data Field: Acceleration Axis Y".to_string(),
            0x455 => "Data Field: Acceleration Axis Z".to_string(),
            0x456 => "Data Field: Angular Velocity".to_string(),
            0x457 => "Data Field: Angular Velocity about X Axis".to_string(),
            0x458 => "Data Field: Angular Velocity about Y Axis".to_string(),
            0x459 => "Data Field: Angular Velocity about Z Axis".to_string(),
            0x45A => "Data Field: Angular Position".to_string(),
            0x45B => "Data Field: Angular Position about X Axis".to_string(),
            0x45C => "Data Field: Angular Position about Y Axis".to_string(),
            0x45D => "Data Field: Angular Position about Z Axis".to_string(),
            0x45E => "Data Field: Motion Speed".to_string(),
            0x45F => "Data Field: Motion Intensity".to_string(),
            0x460..=0x46F => "Reserved".to_string(),
            0x470 => "Data Field: Orientation".to_string(),
            0x471 => "Data Field: Heading".to_string(),
            0x472 => "Data Field: Heading X Axis".to_string(),
            0x473 => "Data Field: Heading Y Axis".to_string(),
            0x474 => "Data Field: Heading Z Axis".to_string(),
            0x475 => "Data Field: Heading Compensated Magnetic North".to_string(),
            0x476 => "Data Field: Heading Compensated True North".to_string(),
            0x477 => "Data Field: Heading Magnetic North".to_string(),
            0x478 => "Data Field: Heading True North".to_string(),
            0x479 => "Data Field: Distance".to_string(),
            0x47A => "Data Field: Distance X Axis".to_string(),
            0x47B => "Data Field: Distance Y Axis".to_string(),
            0x47C => "Data Field: Distance Z Axis".to_string(),
            0x47D => "Data Field: Distance Out-of-Range".to_string(),
            0x47E => "Data Field: Tilt".to_string(),
            0x47F => "Data Field: Tilt X Axis".to_string(),
            0x480 => "Data Field: Tilt Y Axis".to_string(),
            0x481 => "Data Field: Tilt Z Axis".to_string(),
            0x482 => "Data Field: Rotation Matrix".to_string(),
            0x483 => "Data Field: Quaternion".to_string(),
            0x484 => "Data Field: Magnetic Flux".to_string(),
            0x485 => "Data Field: Magnetic Flux X Axis".to_string(),
            0x486 => "Data Field: Magnetic Flux Y Axis".to_string(),
            0x487 => "Data Field: Magnetic Flux Z Axis".to_string(),
            0x488 => "Data Field: Magnetometer Accuracy".to_string(),
            0x489 => "Data Field: Simple Orientation Direction".to_string(),
            0x48A..=0x48F => "Reserved".to_string(),
            0x490 => "Data Field: Mechanical".to_string(),
            0x491 => "Data Field: Boolean Switch State".to_string(),
            0x492 => "Data Field: Boolean Switch Array States".to_string(),
            0x493 => "Data Field: Multivalue Switch Value".to_string(),
            0x494 => "Data Field: Force".to_string(),
            0x495 => "Data Field: Absolute Pressure".to_string(),
            0x496 => "Data Field: Gauge Pressure".to_string(),
            0x497 => "Data Field: Strain".to_string(),
            0x498 => "Data Field: Weight".to_string(),
            0x499..=0x49F => "Reserved".to_string(),
            0x4A0 => "Property: Mechanical".to_string(),
            0x4A1 => "Property: Vibration State".to_string(),
            0x4A2 => "Property: Forward Vibration Speed".to_string(),
            0x4A3 => "Property: Backward Vibration Speed".to_string(),
            0x4A4..=0x4AF => "Reserved".to_string(),
            0x4B0 => "Data Field: Biometric".to_string(),
            0x4B1 => "Data Field: Human Presence".to_string(),
            0x4B2 => "Data Field: Human Proximity Range".to_string(),
            0x4B3 => "Data Field: Human Proximity Out of Range".to_string(),
            0x4B4 => "Data Field: Human Touch State".to_string(),
            0x4B5 => "Data Field: Blood Pressure".to_string(),
            0x4B6 => "Data Field: Blood Pressure Diastolic".to_string(),
            0x4B7 => "Data Field: Blood Pressure Systolic".to_string(),
            0x4B8 => "Data Field: Heart Rate".to_string(),
            0x4B9 => "Data Field: Resting Heart Rate".to_string(),
            0x4BA => "Data Field: Heartbeat Interval".to_string(),
            0x4BB => "Data Field: Respiratory Rate".to_string(),
            0x4BC => "Data Field: SpO2".to_string(),
            0x4BD..=0x4CF => "Reserved".to_string(),
            0x4D0 => "Data Field: Light".to_string(),
            0x4D1 => "Data Field: Illuminance".to_string(),
            0x4D2 => "Data Field: Color Temperature".to_string(),
            0x4D3 => "Data Field: Chromaticity".to_string(),
            0x4D4 => "Data Field: Chromaticity X".to_string(),
            0x4D5 => "Data Field: Chromaticity Y".to_string(),
            0x4D6 => "Data Field: Consumer IR Sentence Receive".to_string(),
            0x4D7 => "Data Field: Infrared Light".to_string(),
            0x4D8 => "Data Field: Red Light".to_string(),
            0x4D9 => "Data Field: Green Light".to_string(),
            0x4DA => "Data Field: Blue Light".to_string(),
            0x4DB => "Data Field: Ultraviolet A Light".to_string(),
            0x4DC => "Data Field: Ultraviolet B Light".to_string(),
            0x4DD => "Data Field: Ultraviolet Index".to_string(),
            0x4DE => "Data Field: Near Infrared Light".to_string(),
            0x4DF => "Property: Light".to_string(),
            0x4E0 => "Property: Consumer IR Sentence Send".to_string(),
            0x4E1..=0x4E1 => "Reserved".to_string(),
            0x4E2 => "Property: Auto Brightness Preferred".to_string(),
            0x4E3 => "Property: Auto Color Preferred".to_string(),
            0x4E4..=0x4EF => "Reserved".to_string(),
            0x4F0 => "Data Field: Scanner".to_string(),
            0x4F1 => "Data Field: RFID Tag 40 Bit".to_string(),
            0x4F2 => "Data Field: NFC Sentence Receive".to_string(),
            0x4F3..=0x4F7 => "Reserved".to_string(),
            0x4F8 => "Property: Scanner".to_string(),
            0x4F9 => "Property: NFC Sentence Send".to_string(),
            0x4FA..=0x4FF => "Reserved".to_string(),
            0x500 => "Data Field: Electrical".to_string(),
            0x501 => "Data Field: Capacitance".to_string(),
            0x502 => "Data Field: Current".to_string(),
            0x503 => "Data Field: Electrical Power".to_string(),
            0x504 => "Data Field: Inductance".to_string(),
            0x505 => "Data Field: Resistance".to_string(),
            0x506 => "Data Field: Voltage".to_string(),
            0x507 => "Data Field: Frequency".to_string(),
            0x508 => "Data Field: Period".to_string(),
            0x509 => "Data Field: Percent of Range".to_string(),
            0x50A..=0x51F => "Reserved".to_string(),
            0x520 => "Data Field: Time".to_string(),
            0x521 => "Data Field: Year".to_string(),
            0x522 => "Data Field: Month".to_string(),
            0x523 => "Data Field: Day".to_string(),
            0x524 => "Data Field: Day of Week".to_string(),
            0x525 => "Data Field: Hour".to_string(),
            0x526 => "Data Field: Minute".to_string(),
            0x527 => "Data Field: Second".to_string(),
            0x528 => "Data Field: Millisecond".to_string(),
            0x529 => "Data Field: Timestamp".to_string(),
            0x52A => "Data Field: Julian Day of Year".to_string(),
            0x52B => "Data Field: Time Since System Boot".to_string(),
            0x52C..=0x52F => "Reserved".to_string(),
            0x530 => "Property: Time".to_string(),
            0x531 => "Property: Time Zone Offset from UTC".to_string(),
            0x532 => "Property: Time Zone Name".to_string(),
            0x533 => "Property: Daylight Savings Time Observed".to_string(),
            0x534 => "Property: Time Trim Adjustment".to_string(),
            0x535 => "Property: Arm Alarm".to_string(),
            0x536..=0x53F => "Reserved".to_string(),
            0x540 => "Data Field: Custom".to_string(),
            0x541 => "Data Field: Custom Usage".to_string(),
            0x542 => "Data Field: Custom Boolean Array".to_string(),
            0x543 => "Data Field: Custom Value".to_string(),
            0x544 => "Data Field: Custom Value 1".to_string(),
            0x545 => "Data Field: Custom Value 2".to_string(),
            0x546 => "Data Field: Custom Value 3".to_string(),
            0x547 => "Data Field: Custom Value 4".to_string(),
            0x548 => "Data Field: Custom Value 5".to_string(),
            0x549 => "Data Field: Custom Value 6".to_string(),
            0x54A => "Data Field: Custom Value 7".to_string(),
            0x54B => "Data Field: Custom Value 8".to_string(),
            0x54C => "Data Field: Custom Value 9".to_string(),
            0x54D => "Data Field: Custom Value 10".to_string(),
            0x54E => "Data Field: Custom Value 11".to_string(),
            0x54F => "Data Field: Custom Value 12".to_string(),
            0x550 => "Data Field: Custom Value 13".to_string(),
            0x551 => "Data Field: Custom Value 14".to_string(),
            0x552 => "Data Field: Custom Value 15".to_string(),
            0x553 => "Data Field: Custom Value 16".to_string(),
            0x554 => "Data Field: Custom Value 17".to_string(),
            0x555 => "Data Field: Custom Value 18".to_string(),
            0x556 => "Data Field: Custom Value 19".to_string(),
            0x557 => "Data Field: Custom Value 20".to_string(),
            0x558 => "Data Field: Custom Value 21".to_string(),
            0x559 => "Data Field: Custom Value 22".to_string(),
            0x55A => "Data Field: Custom Value 23".to_string(),
            0x55B => "Data Field: Custom Value 24".to_string(),
            0x55C => "Data Field: Custom Value 25".to_string(),
            0x55D => "Data Field: Custom Value 26".to_string(),
            0x55E => "Data Field: Custom Value 27".to_string(),
            0x55F => "Data Field: Custom Value 28".to_string(),
            0x560 => "Data Field: Generic".to_string(),
            0x561 => "Data Field: Generic GUID or PROPERTYKEY".to_string(),
            0x562 => "Data Field: Generic Category GUID".to_string(),
            0x563 => "Data Field: Generic Type GUID".to_string(),
            0x564 => "Data Field: Generic Event PROPERTYKEY".to_string(),
            0x565 => "Data Field: Generic Property PROPERTYKEY".to_string(),
            0x566 => "Data Field: Generic Data Field PROPERTYKEY".to_string(),
            0x567 => "Data Field: Generic Event".to_string(),
            0x568 => "Data Field: Generic Property".to_string(),
            0x569 => "Data Field: Generic Data Field".to_string(),
            0x56A => "Data Field: Enumerator Table Row Index".to_string(),
            0x56B => "Data Field: Enumerator Table Row Count".to_string(),
            0x56C => "Data Field: Generic GUID or PROPERTYKEY kind".to_string(),
            0x56D => "Data Field: Generic GUID".to_string(),
            0x56E => "Data Field: Generic PROPERTYKEY".to_string(),
            0x56F => "Data Field: Generic Top Level Collection ID".to_string(),
            0x570 => "Data Field: Generic Report ID".to_string(),
            0x571 => "Data Field: Generic Report Item Position Index".to_string(),
            0x572 => "Data Field: Generic Firmware VARTYPE".to_string(),
            0x573 => "Data Field: Generic Unit of Measure".to_string(),
            0x574 => "Data Field: Generic Unit Exponent".to_string(),
            0x575 => "Data Field: Generic Report Size".to_string(),
            0x576 => "Data Field: Generic Report Count".to_string(),
            0x577..=0x57F => "Reserved".to_string(),
            0x580 => "Property: Generic".to_string(),
            0x581 => "Property: Enumerator Table Row Index".to_string(),
            0x582 => "Property: Enumerator Table Row Count".to_string(),
            0x583..=0x58F => "Reserved".to_string(),
            0x590 => "Data Field: Personal Activity".to_string(),
            0x591 => "Data Field: Activity Type".to_string(),
            0x592 => "Data Field: Activity State".to_string(),
            0x593 => "Data Field: Device Position".to_string(),
            0x594 => "Data Field: Step Count".to_string(),
            0x595 => "Data Field: Step Count Reset".to_string(),
            0x596 => "Data Field: Step Duration".to_string(),
            0x597 => "Data Field: Step Type".to_string(),
            0x598..=0x59F => "Reserved".to_string(),
            0x5A0 => "Property: Minimum Activity Detection Interval".to_string(),
            0x5A1 => "Property: Supported Activity Types".to_string(),
            0x5A2 => "Property: Subscribed Activity Types".to_string(),
            0x5A3 => "Property: Supported Step Types".to_string(),
            0x5A4 => "Property: Subscribed Step Types".to_string(),
            0x5A5 => "Property: Floor Height".to_string(),
            0x5A6..=0x5AF => "Reserved".to_string(),
            0x5B0 => "Data Field: Custom Type ID".to_string(),
            0x5B1..=0x5BF => "Reserved".to_string(),
            0x5C0 => "Property: Custom".to_string(),
            0x5C1 => "Property: Custom Value 1".to_string(),
            0x5C2 => "Property: Custom Value 2".to_string(),
            0x5C3 => "Property: Custom Value 3".to_string(),
            0x5C4 => "Property: Custom Value 4".to_string(),
            0x5C5 => "Property: Custom Value 5".to_string(),
            0x5C6 => "Property: Custom Value 6".to_string(),
            0x5C7 => "Property: Custom Value 7".to_string(),
            0x5C8 => "Property: Custom Value 8".to_string(),
            0x5C9 => "Property: Custom Value 9".to_string(),
            0x5CA => "Property: Custom Value 10".to_string(),
            0x5CB => "Property: Custom Value 11".to_string(),
            0x5CC => "Property: Custom Value 12".to_string(),
            0x5CD => "Property: Custom Value 13".to_string(),
            0x5CE => "Property: Custom Value 14".to_string(),
            0x5CF => "Property: Custom Value 15".to_string(),
            0x5D0 => "Property: Custom Value 16".to_string(),
            0x5D1..=0x5DF => "Reserved".to_string(),
            0x5E0 => "Data Field: Hinge".to_string(),
            0x5E1 => "Data Field: Hinge Angle".to_string(),
            0x5E2..=0x5EF => "Reserved".to_string(),
            0x5F0 => "Data Field: Gesture Sensor".to_string(),
            0x5F1 => "Data Field: Gesture State".to_string(),
            0x5F2 => "Data Field: Hinge Fold Initial Angle".to_string(),
            0x5F3 => "Data Field: Hinge Fold Final Angle".to_string(),
            0x5F4 => "Data Field: Hinge Fold Contributing Panel".to_string(),
            0x5F5 => "Data Field: Hinge Fold Type".to_string(),
            0x5F6..=0x7FF => "Reserved".to_string(),
            0x800 => "Sensor State: Undefined".to_string(),
            0x801 => "Sensor State: Ready".to_string(),
            0x802 => "Sensor State: Not Available".to_string(),
            0x803 => "Sensor State: No Data".to_string(),
            0x804 => "Sensor State: Initializing".to_string(),
            0x805 => "Sensor State: Access Denied".to_string(),
            0x806 => "Sensor State: Error".to_string(),
            0x807..=0x80F => "Reserved".to_string(),
            0x810 => "Sensor Event: Unknown".to_string(),
            0x811 => "Sensor Event: State Changed".to_string(),
            0x812 => "Sensor Event: Property Changed".to_string(),
            0x813 => "Sensor Event: Data Updated".to_string(),
            0x814 => "Sensor Event: Poll Response".to_string(),
            0x815 => "Sensor Event: Change Sensitivity".to_string(),
            0x816 => "Sensor Event: Range Maximum Reached".to_string(),
            0x817 => "Sensor Event: Range Minimum Reached".to_string(),
            0x818 => "Sensor Event: High Threshold Cross Upward".to_string(),
            0x819 => "Sensor Event: High Threshold Cross Downward".to_string(),
            0x81A => "Sensor Event: Low Threshold Cross Upward".to_string(),
            0x81B => "Sensor Event: Low Threshold Cross Downward".to_string(),
            0x81C => "Sensor Event: Zero Threshold Cross Upward".to_string(),
            0x81D => "Sensor Event: Zero Threshold Cross Downward".to_string(),
            0x81E => "Sensor Event: Period Exceeded".to_string(),
            0x81F => "Sensor Event: Frequency Exceeded".to_string(),
            0x820 => "Sensor Event: Complex Trigger".to_string(),
            0x821..=0x82F => "Reserved".to_string(),
            0x830 => "Connection Type: PC Integrated".to_string(),
            0x831 => "Connection Type: PC Attached".to_string(),
            0x832 => "Connection Type: PC External".to_string(),
            0x833..=0x83F => "Reserved".to_string(),
            0x840 => "Reporting State: Report No Events".to_string(),
            0x841 => "Reporting State: Report All Events".to_string(),
            0x842 => "Reporting State: Report Threshold Events".to_string(),
            0x843 => "Reporting State: Wake On No Events".to_string(),
            0x844 => "Reporting State: Wake On All Events".to_string(),
            0x845 => "Reporting State: Wake On Threshold Events".to_string(),
            0x846..=0x84F => "Reserved".to_string(),
            0x850 => "Power State: Undefined".to_string(),
            0x851 => "Power State: D0 Full Power".to_string(),
            0x852 => "Power State: D1 Low Power".to_string(),
            0x853 => "Power State: D2 Standby Power with Wakeup".to_string(),
            0x854 => "Power State: D3 Sleep with Wakeup".to_string(),
            0x855 => "Power State: D4 Power Off".to_string(),
            0x856..=0x85F => "Reserved".to_string(),
            0x860 => "Accuracy: Default".to_string(),
            0x861 => "Accuracy: High".to_string(),
            0x862 => "Accuracy: Medium".to_string(),
            0x863 => "Accuracy: Low".to_string(),
            0x864..=0x86F => "Reserved".to_string(),
            0x870 => "Fix Quality: No Fix".to_string(),
            0x871 => "Fix Quality: GPS".to_string(),
            0x872 => "Fix Quality: DGPS".to_string(),
            0x873..=0x87F => "Reserved".to_string(),
            0x880 => "Fix Type: No Fix".to_string(),
            0x881 => "Fix Type: GPS SPS Mode, Fix Valid".to_string(),
            0x882 => "Fix Type: DGPS SPS Mode, Fix Valid".to_string(),
            0x883 => "Fix Type: GPS PPS Mode, Fix Valid".to_string(),
            0x884 => "Fix Type: Real Time Kinematic".to_string(),
            0x885 => "Fix Type: Float RTK".to_string(),
            0x886 => "Fix Type: Estimated (dead reckoned)".to_string(),
            0x887 => "Fix Type: Manual Input Mode".to_string(),
            0x888 => "Fix Type: Simulator Mode".to_string(),
            0x889..=0x88F => "Reserved".to_string(),
            0x890 => "GPS Operation Mode: Manual".to_string(),
            0x891 => "GPS Operation Mode: Automatic".to_string(),
            0x892..=0x89F => "Reserved".to_string(),
            0x8A0 => "GPS Selection Mode: Autonomous".to_string(),
            0x8A1 => "GPS Selection Mode: DGPS".to_string(),
            0x8A2 => "GPS Selection Mode: Estimated (dead reckoned)".to_string(),
            0x8A3 => "GPS Selection Mode: Manual Input".to_string(),
            0x8A4 => "GPS Selection Mode: Simulator".to_string(),
            0x8A5 => "GPS Selection Mode: Data Not Valid".to_string(),
            0x8A6..=0x8AF => "Reserved".to_string(),
            0x8B0 => "GPS Status Data: Valid".to_string(),
            0x8B1 => "GPS Status Data: Not Valid".to_string(),
            0x8B2..=0x8BF => "Reserved".to_string(),
            0x8C0 => "Day of Week: Sunday".to_string(),
            0x8C1 => "Day of Week: Monday".to_string(),
            0x8C2 => "Day of Week: Tuesday".to_string(),
            0x8C3 => "Day of Week: Wednesday".to_string(),
            0x8C4 => "Day of Week: Thursday".to_string(),
            0x8C5 => "Day of Week: Friday".to_string(),
            0x8C6 => "Day of Week: Saturday".to_string(),
            0x8C7..=0x8CF => "Reserved".to_string(),
            0x8D0 => "Kind: Category".to_string(),
            0x8D1 => "Kind: Type".to_string(),
            0x8D2 => "Kind: Event".to_string(),
            0x8D3 => "Kind: Property".to_string(),
            0x8D4 => "Kind: Data Field".to_string(),
            0x8D5..=0x8DF => "Reserved".to_string(),
            0x8E0 => "Magnetometer Accuracy: Low".to_string(),
            0x8E1 => "Magnetometer Accuracy: Medium".to_string(),
            0x8E2 => "Magnetometer Accuracy: High".to_string(),
            0x8E3..=0x8EF => "Reserved".to_string(),
            0x8F0 => "Simple Orientation Direction: Not Rotated".to_string(),
            0x8F1 => "Simple Orientation Direction: Rotated 90 Degrees CCW".to_string(),
            0x8F2 => "Simple Orientation Direction: Rotated 180 Degrees CCW".to_string(),
            0x8F3 => "Simple Orientation Direction: Rotated 270 Degrees CCW".to_string(),
            0x8F4 => "Simple Orientation Direction: Face Up".to_string(),
            0x8F5 => "Simple Orientation Direction: Face Down".to_string(),
            0x8F6..=0x8FF => "Reserved".to_string(),
            0x900 => "VT_NULL".to_string(),
            0x901 => "VT_BOOL".to_string(),
            0x902 => "VT_UI1".to_string(),
            0x903 => "VT_I1".to_string(),
            0x904 => "VT_UI2".to_string(),
            0x905 => "VT_I2".to_string(),
            0x906 => "VT_UI4".to_string(),
            0x907 => "VT_I4".to_string(),
            0x908 => "VT_UI8".to_string(),
            0x909 => "VT_I8".to_string(),
            0x90A => "VT_R4".to_string(),
            0x90B => "VT_R8".to_string(),
            0x90C => "VT_WSTR".to_string(),
            0x90D => "VT_STR".to_string(),
            0x90E => "VT_CLSID".to_string(),
            0x90F => "VT_VECTOR VT_UI1".to_string(),
            0x910 => "VT_F16E0".to_string(),
            0x911 => "VT_F16E1".to_string(),
            0x912 => "VT_F16E2".to_string(),
            0x913 => "VT_F16E3".to_string(),
            0x914 => "VT_F16E4".to_string(),
            0x915 => "VT_F16E5".to_string(),
            0x916 => "VT_F16E6".to_string(),
            0x917 => "VT_F16E7".to_string(),
            0x918 => "VT_F16E8".to_string(),
            0x919 => "VT_F16E9".to_string(),
            0x91A => "VT_F16EA".to_string(),
            0x91B => "VT_F16EB".to_string(),
            0x91C => "VT_F16EC".to_string(),
            0x91D => "VT_F16ED".to_string(),
            0x91E => "VT_F16EE".to_string(),
            0x91F => "VT_F16EF".to_string(),
            0x920 => "VT_F32E0".to_string(),
            0x921 => "VT_F32E1".to_string(),
            0x922 => "VT_F32E2".to_string(),
            0x923 => "VT_F32E3".to_string(),
            0x924 => "VT_F32E4".to_string(),
            0x925 => "VT_F32E5".to_string(),
            0x926 => "VT_F32E6".to_string(),
            0x927 => "VT_F32E7".to_string(),
            0x928 => "VT_F32E8".to_string(),
            0x929 => "VT_F32E9".to_string(),
            0x92A => "VT_F32EA".to_string(),
            0x92B => "VT_F32EB".to_string(),
            0x92C => "VT_F32EC".to_string(),
            0x92D => "VT_F32ED".to_string(),
            0x92E => "VT_F32EE".to_string(),
            0x92F => "VT_F32EF".to_string(),
            0x930 => "Activity Type: Unknown".to_string(),
            0x931 => "Activity Type: Stationary".to_string(),
            0x932 => "Activity Type: Fidgeting".to_string(),
            0x933 => "Activity Type: Walking".to_string(),
            0x934 => "Activity Type: Running".to_string(),
            0x935 => "Activity Type: In Vehicle".to_string(),
            0x936 => "Activity Type: Biking".to_string(),
            0x937 => "Activity Type: Idle".to_string(),
            0x938..=0x93F => "Reserved".to_string(),
            0x940 => "Unit: Not Specified".to_string(),
            0x941 => "Unit: Lux".to_string(),
            0x942 => "Unit: Degrees Kelvin".to_string(),
            0x943 => "Unit: Degrees Celsius".to_string(),
            0x944 => "Unit: Pascal".to_string(),
            0x945 => "Unit: Newton".to_string(),
            0x946 => "Unit: Meters/Second".to_string(),
            0x947 => "Unit: Kilogram".to_string(),
            0x948 => "Unit: Meter".to_string(),
            0x949 => "Unit: Meters/Second/Second".to_string(),
            0x94A => "Unit: Farad".to_string(),
            0x94B => "Unit: Ampere".to_string(),
            0x94C => "Unit: Watt".to_string(),
            0x94D => "Unit: Henry".to_string(),
            0x94E => "Unit: Ohm".to_string(),
            0x94F => "Unit: Volt".to_string(),
            0x950 => "Unit: Hertz".to_string(),
            0x951 => "Unit: Bar".to_string(),
            0x952 => "Unit: Degrees Anti-clockwise".to_string(),
            0x953 => "Unit: Degrees Clockwise".to_string(),
            0x954 => "Unit: Degrees".to_string(),
            0x955 => "Unit: Degrees/Second".to_string(),
            0x956 => "Unit: Degrees/Second/Second".to_string(),
            0x957 => "Unit: Knot".to_string(),
            0x958 => "Unit: Percent".to_string(),
            0x959 => "Unit: Second".to_string(),
            0x95A => "Unit: Millisecond".to_string(),
            0x95B => "Unit: G".to_string(),
            0x95C => "Unit: Bytes".to_string(),
            0x95D => "Unit: Milligauss".to_string(),
            0x95E => "Unit: Bits".to_string(),
            0x95F..=0x95F => "Reserved".to_string(),
            0x960 => "Activity State: No State Change".to_string(),
            0x961 => "Activity State: Start Activity".to_string(),
            0x962 => "Activity State: End Activity".to_string(),
            0x963..=0x96F => "Reserved".to_string(),
            0x970 => "Exponent 0".to_string(),
            0x971 => "Exponent 1".to_string(),
            0x972 => "Exponent 2".to_string(),
            0x973 => "Exponent 3".to_string(),
            0x974 => "Exponent 4".to_string(),
            0x975 => "Exponent 5".to_string(),
            0x976 => "Exponent 6".to_string(),
            0x977 => "Exponent 7".to_string(),
            0x978 => "Exponent 8".to_string(),
            0x979 => "Exponent 9".to_string(),
            0x97A => "Exponent A".to_string(),
            0x97B => "Exponent B".to_string(),
            0x97C => "Exponent C".to_string(),
            0x97D => "Exponent D".to_string(),
            0x97E => "Exponent E".to_string(),
            0x97F => "Exponent F".to_string(),
            0x980 => "Device Position: Unknown".to_string(),
            0x981 => "Device Position: Unchanged".to_string(),
            0x982 => "Device Position: On Desk".to_string(),
            0x983 => "Device Position: In Hand".to_string(),
            0x984 => "Device Position: Moving in Bag".to_string(),
            0x985 => "Device Position: Stationary in Bag".to_string(),
            0x986..=0x98F => "Reserved".to_string(),
            0x990 => "Step Type: Unknown".to_string(),
            0x991 => "Step Type: Running".to_string(),
            0x992 => "Step Type: Walking".to_string(),
            0x993..=0x99F => "Reserved".to_string(),
            0x9A0 => "Gesture State: Unknown".to_string(),
            0x9A1 => "Gesture State: Started".to_string(),
            0x9A2 => "Gesture State: Completed".to_string(),
            0x9A3 => "Gesture State: Cancelled".to_string(),
            0x9A4..=0x9AF => "Reserved".to_string(),
            0x9B0 => "Hinge Fold Contributing Panel: Unknown".to_string(),
            0x9B1 => "Hinge Fold Contributing Panel: Panel 1".to_string(),
            0x9B2 => "Hinge Fold Contributing Panel: Panel 2".to_string(),
            0x9B3 => "Hinge Fold Contributing Panel: Both".to_string(),
            0x9B4 => "Hinge Fold Type: Unknown".to_string(),
            0x9B5 => "Hinge Fold Type: Increasing".to_string(),
            0x9B6 => "Hinge Fold Type: Decreasing".to_string(),
            0x9B7..=0xFFF => "Reserved".to_string(),
            0x1000 => "Modifier: Change Sensitivity Absolute".to_string(),
            0x1001..=0x10FF => "Reserved".to_string(),
            0x1100..=0x17FF => "Reserved for use as Change Sensitivity Absolute modifier range".to_string(),
            0x1800..=0x1FFF => "Reserved".to_string(),
            0x2000 => "Modifier: Maximum".to_string(),
            0x2001..=0x20FF => "Reserved".to_string(),
            0x2100..=0x27FF => "Reserved for use as Maximum modifier range".to_string(),
            0x2800..=0x2FFF => "Reserved".to_string(),
            0x3000 => "Modifier: Minimum".to_string(),
            0x3001..=0x30FF => "Reserved".to_string(),
            0x3100..=0x37FF => "Reserved for use as Minimum modifier range".to_string(),
            0x3800..=0x3FFF => "Reserved".to_string(),
            0x4000 => "Modifier: Accuracy".to_string(),
            0x4001..=0x40FF => "Reserved".to_string(),
            0x4100..=0x47FF => "Reserved for use as Accuracy modifier range".to_string(),
            0x4800..=0x4FFF => "Reserved".to_string(),
            0x5000 => "Modifier: Resolution".to_string(),
            0x5001..=0x50FF => "Reserved".to_string(),
            0x5100..=0x57FF => "Reserved for use as Resolution modifier range".to_string(),
            0x5800..=0x5FFF => "Reserved".to_string(),
            0x6000 => "Modifier: Threshold High".to_string(),
            0x6001..=0x60FF => "Reserved".to_string(),
            0x6100..=0x67FF => "Reserved for use as Threshold High modifier range".to_string(),
            0x6800..=0x6FFF => "Reserved".to_string(),
            0x7000 => "Modifier: Threshold Low".to_string(),
            0x7001..=0x70FF => "Reserved".to_string(),
            0x7100..=0x77FF => "Reserved for use as Threshold Low modifier range".to_string(),
            0x7800..=0x7FFF => "Reserved".to_string(),
            0x8000 => "Modifier: Calibration Offset".to_string(),
            0x8001..=0x80FF => "Reserved".to_string(),
            0x8100..=0x87FF => "Reserved for use as Calibration Offset modifier range".to_string(),
            0x8800..=0x8FFF => "Reserved".to_string(),
            0x9000 => "Modifier: Calibration Multiplier".to_string(),
            0x9001..=0x90FF => "Reserved".to_string(),
            0x9100..=0x97FF => "Reserved for use as Calibration Multiplier modifier range".to_string(),
            0x9800..=0x9FFF => "Reserved".to_string(),
            0xA000 => "Modifier: Report Interval".to_string(),
            0xA001..=0xA0FF => "Reserved".to_string(),
            0xA100..=0xA7FF => "Reserved for use as Report Interval modifier range".to_string(),
            0xA800..=0xAFFF => "Reserved".to_string(),
            0xB000 => "Modifier: Frequency Max".to_string(),
            0xB001..=0xB0FF => "Reserved".to_string(),
            0xB100..=0xB7FF => "Reserved for use as Frequency Max modifier range".to_string(),
            0xB800..=0xBFFF => "Reserved".to_string(),
            0xC000 => "Modifier: Period Max".to_string(),
            0xC001..=0xC0FF => "Reserved".to_string(),
            0xC100..=0xC7FF => "Reserved for use as Period Max modifier range".to_string(),
            0xC800..=0xCFFF => "Reserved".to_string(),
            0xD000 => "Modifier: Change Sensitivity Percent of Range".to_string(),
            0xD001..=0xD0FF => "Reserved".to_string(),
            0xD100..=0xD7FF => "Reserved for use as Change Sensitivity Percent modifier range".to_string(),
            0xD800..=0xDFFF => "Reserved".to_string(),
            0xE000 => "Modifier: Change Sensitivity Percent Relative".to_string(),
            0xE001..=0xE0FF => "Reserved".to_string(),
            0xE100..=0xE7FF => "Reserved for use as Change Sensitivity Percent modifier range".to_string(),
            0xE800..=0xEFFF => "Reserved".to_string(),
            0xF000 => "Modifier: Vendor Reserved".to_string(),
            0xF001..=0xF0FF => "Reserved".to_string(),
            0xF100..=0xF7FF => "Reserved for use as Vendor Reserved modifier range".to_string(),
            0xF800..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_medical_instrument_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Medical Ultrasound".to_string(),
            0x02..=0x1F => "Reserved".to_string(),
            0x20 => "VCR/Acquisition".to_string(),
            0x21 => "Freeze/Thaw".to_string(),
            0x22 => "Clip Store".to_string(),
            0x23 => "Update".to_string(),
            0x24 => "Next".to_string(),
            0x25 => "Save".to_string(),
            0x26 => "Print".to_string(),
            0x27 => "Microphone Enable".to_string(),
            0x28..=0x3F => "Reserved".to_string(),
            0x40 => "Cine".to_string(),
            0x41 => "Transmit Power".to_string(),
            0x42 => "Volume".to_string(),
            0x43 => "Focus".to_string(),
            0x44 => "Depth".to_string(),
            0x45..=0x5F => "Reserved".to_string(),
            0x60 => "Soft Step - Primary".to_string(),
            0x61 => "Soft Step - Secondary".to_string(),
            0x62..=0x6F => "Reserved".to_string(),
            0x70 => "Depth Gain Compensation".to_string(),
            0x71..=0x7F => "Reserved".to_string(),
            0x80 => "Zoom Select".to_string(),
            0x81 => "Zoom Adjust".to_string(),
            0x82 => "Spectral Doppler Mode Select".to_string(),
            0x83 => "Spectral Doppler Adjust".to_string(),
            0x84 => "Color Doppler Mode Select".to_string(),
            0x85 => "Color Doppler Adjust".to_string(),
            0x86 => "Motion Mode Select".to_string(),
            0x87 => "Motion Mode Adjust".to_string(),
            0x88 => "2-D Mode Select".to_string(),
            0x89 => "2-D Mode Adjust".to_string(),
            0x8A..=0x9F => "Reserved".to_string(),
            0xA0 => "Soft Control Select".to_string(),
            0xA1 => "Soft Control Adjust".to_string(),
            0xA2..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_braille_display_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Braille Display".to_string(),
            0x02 => "Braille Row".to_string(),
            0x03 => "8 Dot Braille Cell".to_string(),
            0x04 => "6 Dot Braille Cell".to_string(),
            0x05 => "Number of Braille Cells".to_string(),
            0x06 => "Screen Reader Control".to_string(),
            0x07 => "Screen Reader Identifier".to_string(),
            0x08..=0xF9 => "Reserved".to_string(),
            0xFA => "Router Set 1".to_string(),
            0xFB => "Router Set 2".to_string(),
            0xFC => "Router Set 3".to_string(),
            0xFD..=0xFF => "Reserved".to_string(),
            0x100 => "Router Key".to_string(),
            0x101 => "Row Router Key".to_string(),
            0x102..=0x1FF => "Reserved".to_string(),
            0x200 => "Braille Buttons".to_string(),
            0x201 => "Braille Keyboard Dot 1".to_string(),
            0x202 => "Braille Keyboard Dot 2".to_string(),
            0x203 => "Braille Keyboard Dot 3".to_string(),
            0x204 => "Braille Keyboard Dot 4".to_string(),
            0x205 => "Braille Keyboard Dot 5".to_string(),
            0x206 => "Braille Keyboard Dot 6".to_string(),
            0x207 => "Braille Keyboard Dot 7".to_string(),
            0x208 => "Braille Keyboard Dot 8".to_string(),
            0x209 => "Braille Keyboard Space".to_string(),
            0x20A => "Braille Keyboard Left Space".to_string(),
            0x20B => "Braille Keyboard Right Space".to_string(),
            0x20C => "Braille Face Controls".to_string(),
            0x20D => "Braille Left Controls".to_string(),
            0x20E => "Braille Right Controls".to_string(),
            0x20F => "Braille Top Controls".to_string(),
            0x210 => "Braille Joystick Center".to_string(),
            0x211 => "Braille Joystick Up".to_string(),
            0x212 => "Braille Joystick Down".to_string(),
            0x213 => "Braille Joystick Left".to_string(),
            0x214 => "Braille Joystick Right".to_string(),
            0x215 => "Braille D-Pad Center".to_string(),
            0x216 => "Braille D-Pad Up".to_string(),
            0x217 => "Braille D-Pad Down".to_string(),
            0x218 => "Braille D-Pad Left".to_string(),
            0x219 => "Braille D-Pad Right".to_string(),
            0x21A => "Braille Pan Left".to_string(),
            0x21B => "Braille Pan Right".to_string(),
            0x21C => "Braille Rocker Up".to_string(),
            0x21D => "Braille Rocker Down".to_string(),
            0x21E => "Braille Rocker Press".to_string(),
            0x21F..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_lightning_and_illumination_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "LampArray".to_string(),
            0x02 => "LampArrayAttributesReport".to_string(),
            0x03 => "LampCount".to_string(),
            0x04 => "BoundingBoxWidthInMicrometers".to_string(),
            0x05 => "BoundingBoxHeightInMicrometers".to_string(),
            0x06 => "BoundingBoxDepthInMicrometers".to_string(),
            0x07 => "LampArrayKind".to_string(),
            0x08 => "MinUpdateIntervalInMicroseconds".to_string(),
            0x09..=0x1F => "Reserved".to_string(),
            0x20 => "LampAttributesRequestReport".to_string(),
            0x21 => "LampId".to_string(),
            0x22 => "LampAttributesResponseReport".to_string(),
            0x23 => "PositionXInMicrometers".to_string(),
            0x24 => "PositionYInMicrometers".to_string(),
            0x25 => "PositionZInMicrometers".to_string(),
            0x26 => "LampPurposes".to_string(),
            0x27 => "UpdateLatencyInMicroseconds".to_string(),
            0x28 => "RedLevelCount".to_string(),
            0x29 => "GreenLevelCount".to_string(),
            0x2A => "BlueLevelCount".to_string(),
            0x2B => "IntensityLevelCount".to_string(),
            0x2C => "IsProgrammable".to_string(),
            0x2D => "InputBinding".to_string(),
            0x2E..=0x4F => "Reserved".to_string(),
            0x50 => "LampMultiUpdateReport".to_string(),
            0x51 => "RedUpdateChannel".to_string(),
            0x52 => "GreenUpdateChannel".to_string(),
            0x53 => "BlueUpdateChannel".to_string(),
            0x54 => "IntensityUpdateChannel".to_string(),
            0x55 => "LampUpdateFlags".to_string(),
            0x56..=0x5F => "Reserved".to_string(),
            0x60 => "LampRangeUpdateReport".to_string(),
            0x61 => "LampIdStart".to_string(),
            0x62 => "LampIdEnd".to_string(),
            0x63..=0x6F => "Reserved".to_string(),
            0x70 => "LampArrayControlReport".to_string(),
            0x71 => "AutonomousMode".to_string(),
            0x72..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_monitor_usage_name(usage_page: u32, usage: u32) -> String {
        // 0x80..=0x83
        match usage {
            _ => format!("{:#04X}-{:#04X}", usage_page, usage),
        }
    }

    fn get_power_usage_name(usage_page: u32, usage: u32) -> String {
        // 0x84..=0x87
        match usage {
            _ => format!("{:#04X}-{:#04X}", usage_page, usage),
        }
    }

    fn get_bar_code_scanner_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Bar Code Badge Reader".to_string(),
            0x02 => "Bar Code Scanner".to_string(),
            0x03 => "Dumb Bar Code Scanner".to_string(),
            0x04 => "Cordless Scanner Base".to_string(),
            0x05 => "Bar Code Scanner Cradle".to_string(),
            0x06..=0x0F => "Reserved".to_string(),
            0x10 => "Attribute Report".to_string(),
            0x11 => "Settings Report".to_string(),
            0x12 => "Scanned Data Report".to_string(),
            0x13 => "Raw Scanned Data Report".to_string(),
            0x14 => "Trigger Report".to_string(),
            0x15 => "Status Report".to_string(),
            0x16 => "UPC/EAN Control Report".to_string(),
            0x17 => "EAN 2/3 Label Control Report".to_string(),
            0x18 => "Code 39 Control Report".to_string(),
            0x19 => "Interleaved 2 of 5 Control Report".to_string(),
            0x1A => "Standard 2 of 5 Control Report".to_string(),
            0x1B => "MSI Plessey Control Report".to_string(),
            0x1C => "Codabar Control Report".to_string(),
            0x1D => "Code 128 Control Report".to_string(),
            0x1E => "Misc 1D Control Report".to_string(),
            0x1F => "2D Control Report".to_string(),
            0x20..=0x2F => "Reserved".to_string(),
            0x30 => "Aiming/Pointer Mode".to_string(),
            0x31 => "Bar Code Present Sensor".to_string(),
            0x32 => "Class 1A Laser".to_string(),
            0x33 => "Class 2 Laser".to_string(),
            0x34 => "Heater Present".to_string(),
            0x35 => "Contact Scanner".to_string(),
            0x36 => "Electronic Article Surveillance Notification".to_string(),
            0x37 => "Constant Electronic Article Surveillance".to_string(),
            0x38 => "Error Indication".to_string(),
            0x39 => "Fixed Beeper".to_string(),
            0x3A => "Good Decode Indication".to_string(),
            0x3B => "Hands Free Scanning".to_string(),
            0x3C => "Intrinsically Safe".to_string(),
            0x3D => "Klasse Eins Laser".to_string(),
            0x3E => "Long Range Scanner".to_string(),
            0x3F => "Mirror Speed Control".to_string(),
            0x40 => "Not On File Indication".to_string(),
            0x41 => "Programmable Beeper".to_string(),
            0x42 => "Triggerless".to_string(),
            0x43 => "Wand".to_string(),
            0x44 => "Water Resistant".to_string(),
            0x45 => "Multi-Range Scanner".to_string(),
            0x46 => "Proximity Sensor".to_string(),
            0x47..=0x4C => "Reserved".to_string(),
            0x4D => "Fragment Decoding".to_string(),
            0x4E => "Scanner Read Confidence".to_string(),
            0x4F => "Data Prefix".to_string(),
            0x50 => "Prefix AIMI".to_string(),
            0x51 => "Prefix None".to_string(),
            0x52 => "Prefix Proprietary".to_string(),
            0x53..=0x54 => "Reserved".to_string(),
            0x55 => "Active Time".to_string(),
            0x56 => "Aiming Laser Pattern".to_string(),
            0x57 => "Bar Code Present".to_string(),
            0x58 => "Beeper State".to_string(),
            0x59 => "Laser On Time".to_string(),
            0x5A => "Laser State".to_string(),
            0x5B => "Lockout Time".to_string(),
            0x5C => "Motor State".to_string(),
            0x5D => "Motor Timeout".to_string(),
            0x5E => "Power On Reset Scanner".to_string(),
            0x5F => "Prevent Read of Barcodes".to_string(),
            0x60 => "Initiate Barcode Read".to_string(),
            0x61 => "Trigger State".to_string(),
            0x62 => "Trigger Mode".to_string(),
            0x63 => "Trigger Mode Blinking Laser On".to_string(),
            0x64 => "Trigger Mode Continuous Laser On".to_string(),
            0x65 => "Trigger Mode Laser on while Pulled".to_string(),
            0x66 => "Trigger Mode Laser stays on after Trigger release".to_string(),
            0x67..=0x6C => "Reserved".to_string(),
            0x6D => "Commit Parameters to NVM".to_string(),
            0x6E => "Parameter Scanning".to_string(),
            0x6F => "Parameters Changed".to_string(),
            0x70 => "Set parameter default values".to_string(),
            0x71..=0x74 => "Reserved".to_string(),
            0x75 => "Scanner In Cradle".to_string(),
            0x76 => "Scanner In Range".to_string(),
            0x77..=0x79 => "Reserved".to_string(),
            0x7A => "Aim Duration".to_string(),
            0x7B => "Good Read Lamp Duration".to_string(),
            0x7C => "Good Read Lamp Intensity".to_string(),
            0x7D => "Good Read LED".to_string(),
            0x7E => "Good Read Tone Frequency".to_string(),
            0x7F => "Good Read Tone Length".to_string(),
            0x80 => "Good Read Tone Volume".to_string(),
            0x81 => "Reserved".to_string(),
            0x82 => "No Read Message".to_string(),
            0x83 => "Not on File Volume".to_string(),
            0x84 => "Powerup Beep".to_string(),
            0x85 => "Sound Error Beep".to_string(),
            0x86 => "Sound Good Read Beep".to_string(),
            0x87 => "Sound Not On File Beep".to_string(),
            0x88 => "Good Read When to Write".to_string(),
            0x89 => "GRWTI After Decode".to_string(),
            0x8A => "GRWTI Beep/Lamp after transmit".to_string(),
            0x8B => "GRWTI No Beep/Lamp use at all".to_string(),
            0x8C..=0x90 => "Reserved".to_string(),
            0x91 => "Bookland EAN".to_string(),
            0x92 => "Convert EAN 8 to 13 Type".to_string(),
            0x93 => "Convert UPC A to EAN-13".to_string(),
            0x94 => "Convert UPC-E to A".to_string(),
            0x95 => "EAN-13".to_string(),
            0x96 => "EAN-8".to_string(),
            0x97 => "EAN-99 128_Mandatory".to_string(),
            0x98 => "EAN-99 P5/128_Optional".to_string(),
            0x99 => "Reserved".to_string(),
            0x9A => "UPC/EAN".to_string(),
            0x9B => "UPC/EAN Coupon Code".to_string(),
            0x9C => "UPC/EAN Periodicals".to_string(),
            0x9D => "UPC-A".to_string(),
            0x9E => "UPC-A with 128 Mandatory".to_string(),
            0x9F => "UPC-A with 128 Optional".to_string(),
            0xA0 => "UPC-A with P5 Optional".to_string(),
            0xA1 => "UPC-E".to_string(),
            0xA2 => "UPC-E1".to_string(),
            0xA2..=0xA8 => "Reserved".to_string(),
            0xA9 => "Periodical".to_string(),
            0xAA => "Periodical Auto-Discriminate + 2".to_string(),
            0xAB => "Periodical Only Decode with + 2".to_string(),
            0xAC => "Periodical Ignore + 2".to_string(),
            0xAD => "Periodical Auto-Discriminate + 5".to_string(),
            0xAE => "Periodical Only Decode with + 5".to_string(),
            0xAF => "Periodical Ignore + 5".to_string(),
            0xB0 => "Check".to_string(),
            0xB1 => "Check Disable Price".to_string(),
            0xB2 => "Check Enable 4 digit Price".to_string(),
            0xB3 => "Check Enable 5 digit Price".to_string(),
            0xB4 => "Check Enable European 4 digit Price".to_string(),
            0xB5 => "Check Enable European 5 digit Price".to_string(),
            0xB6 => "Reserved".to_string(),
            0xB7 => "EAN Two Label".to_string(),
            0xB8 => "EAN Three Label".to_string(),
            0xB9 => "EAN 8 Flag Digit 1".to_string(),
            0xBA => "EAN 8 Flag Digit 2".to_string(),
            0xBB => "EAN 8 Flag Digit 3".to_string(),
            0xBC => "EAN 13 Flag Digit 1".to_string(),
            0xBD => "EAN 13 Flag Digit 2".to_string(),
            0xBE => "EAN 13 Flag Digit 3".to_string(),
            0xBF => "Add EAN 2/3 Label Definition".to_string(),
            0xC0 => "Clear all EAN 2/3 Label Definitions".to_string(),
            0xC1 => "Reserved".to_string(),
            0xC2 => "Reserved".to_string(),
            0xC3 => "Codabar DF 0".to_string(),
            0xC4 => "Code 128 DF 0".to_string(),
            0xC5 => "Reserved".to_string(),
            0xC6 => "Reserved".to_string(),
            0xC7 => "Code 39 DF 0".to_string(),
            0xC8 => "Code 93 DF 0".to_string(),
            0xC9 => "Full ASCII Conversion DF 0".to_string(),
            0xCA => "Interleaved 2 of 5 DF 0".to_string(),
            0xCB => "Italian Pharmacy Code DF 0".to_string(),
            0xCC => "MSI/Plessey DF 0".to_string(),
            0xCD => "Standard 2 of 5 IATA DF 0".to_string(),
            0xCE => "Standard 2 of 5 DF 0".to_string(),
            0xCF => "Reserved".to_string(),
            0xD0 => "Reserved".to_string(),
            0xD1 => "Reserved".to_string(),
            0xD2 => "Reserved".to_string(),
            0xD3 => "Transmit Start/Stop DF 0".to_string(),
            0xD4 => "Tri-Optic DF 0".to_string(),
            0xD5 => "UCC/EAN-128 DF 0".to_string(),
            0xD6 => "Check Digit".to_string(),
            0xD7 => "Check Digit Disable".to_string(),
            0xD8 => "Check Digit Enable Interleaved 2 of 5 OPCC".to_string(),
            0xD9 => "Check Digit Enable Interleaved 2 of 5 USS".to_string(),
            0xDA => "Check Digit Enable Standard 2 of 5 OPCC".to_string(),
            0xDB => "Check Digit Enable Standard 2 of 5 USS".to_string(),
            0xDC => "Check Digit Enable One MSI Plessey".to_string(),
            0xDD => "Check Digit Enable Two MSI Plessey".to_string(),
            0xDE => "Check Digit Codabar Enable".to_string(),
            0xDF => "Check Digit Code 39 Enable".to_string(),
            0xE0..=0xEF => "Reserved".to_string(),
            0xF0 => "Transmit Check Digit".to_string(),
            0xF1 => "Disable Check Digit Transmit".to_string(),
            0xF2 => "Enable Check Digit Transmit".to_string(),
            0xF3..=0xFA => "Reserved".to_string(),
            0xFB => "Symbology Identifier 1".to_string(),
            0xFC => "Symbology Identifier 2".to_string(),
            0xFD => "Symbology Identifier 3".to_string(),
            0xFE => "Decoded Data".to_string(),
            0xFF => "Decode Data Continued".to_string(),
            0x100 => "Bar Space Data".to_string(),
            0x101 => "Scanner Data Accuracy".to_string(),
            0x102 => "Raw Data Polarity".to_string(),
            0x103 => "Polarity Inverted Bar Code".to_string(),
            0x104 => "Polarity Normal Bar Code".to_string(),
            0x105 => "Reserved".to_string(),
            0x106 => "Minimum Length to Decode".to_string(),
            0x107 => "Maximum Length to Decode".to_string(),
            0x108 => "First Discrete Length to Decode".to_string(),
            0x109 => "Second Discrete Length to Decode".to_string(),
            0x10A => "Data Length Method".to_string(),
            0x10B => "DL Method Read any".to_string(),
            0x10C => "DL Method Check in Range".to_string(),
            0x10D => "DL Method Check for Discrete".to_string(),
            0x10E..=0x10F => "Reserved".to_string(),
            0x110 => "Aztec Code".to_string(),
            0x111 => "BC412".to_string(),
            0x112 => "Channel Code".to_string(),
            0x113 => "Code 16".to_string(),
            0x114 => "Code 32".to_string(),
            0x115 => "Code 49".to_string(),
            0x116 => "Code One".to_string(),
            0x117 => "Colorcode".to_string(),
            0x118 => "Data Matrix".to_string(),
            0x119 => "MaxiCode".to_string(),
            0x11A => "MicroPDF".to_string(),
            0x11B => "PDF-417".to_string(),
            0x11C => "PosiCode".to_string(),
            0x11D => "QR Code".to_string(),
            0x11E => "SuperCode".to_string(),
            0x11F => "UltraCode".to_string(),
            0x120 => "USD-5 (Slug Code)".to_string(),
            0x121 => "VeriCode".to_string(),
            0x122..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_scale_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "Weighing Device".to_string(),
            0x01..=0x1F => "Reserved".to_string(),
            0x20 => "Scale Device".to_string(),
            0x21 => "Scale Class I Metric".to_string(),
            0x22 => "Scale Class I Metric".to_string(),
            0x23 => "Scale Class II Metric".to_string(),
            0x24 => "Scale Class III Metric".to_string(),
            0x25 => "Scale Class IIIL Metric".to_string(),
            0x26 => "Scale Class IV Metric".to_string(),
            0x27 => "Scale Class III English".to_string(),
            0x28 => "Scale Class IIIL English".to_string(),
            0x29 => "Scale Class IV English".to_string(),
            0x2A => "Scale Class Generic".to_string(),
            0x2B..=0x2F => "Reserved".to_string(),
            0x30 => "Scale Attribute Report".to_string(),
            0x31 => "Scale Control Report".to_string(),
            0x32 => "Scale Data Report".to_string(),
            0x33 => "Scale Status Report".to_string(),
            0x34 => "Scale Weight Limit Report".to_string(),
            0x35 => "Scale Statistics Report".to_string(),
            0x36..=0x3F => "Reserved".to_string(),
            0x40 => "Data Weight".to_string(),
            0x41 => "Data Scaling".to_string(),
            0x42..=0x4F => "Reserved".to_string(),
            0x50 => "Weight Unit".to_string(),
            0x51 => "Weight Unit Milligram".to_string(),
            0x52 => "Weight Unit Gram".to_string(),
            0x53 => "Weight Unit Kilogram".to_string(),
            0x54 => "Weight Unit Carats".to_string(),
            0x55 => "Weight Unit Taels".to_string(),
            0x56 => "Weight Unit Grains".to_string(),
            0x57 => "Weight Unit Pennyweights".to_string(),
            0x58 => "Weight Unit Metric Ton".to_string(),
            0x59 => "Weight Unit Avoir Ton".to_string(),
            0x5A => "Weight Unit Troy Ounce".to_string(),
            0x5B => "Weight Unit Ounce".to_string(),
            0x5C => "Weight Unit Pound".to_string(),
            0x5D..=0x5F => "Reserved".to_string(),
            0x60 => "Calibration Count".to_string(),
            0x61 => "Re-Zero Count".to_string(),
            0x62..=0x6F => "Reserved".to_string(),
            0x70 => "Scale Status".to_string(),
            0x71 => "Scale Status Fault".to_string(),
            0x72 => "Scale Status Stable at Center of Zero".to_string(),
            0x73 => "Scale Status In Motion".to_string(),
            0x74 => "Scale Status Weight Stable".to_string(),
            0x75 => "Scale Status Under Zero".to_string(),
            0x76 => "Scale Status Over Weight Limit".to_string(),
            0x77 => "Scale Status Requires Calibration".to_string(),
            0x78 => "Scale Status Requires Re- zeroing".to_string(),
            0x79..=0x7F => "Reserved".to_string(),
            0x80 => "Zero Scale".to_string(),
            0x81 => "Enforced Zero Return".to_string(),
            0x82..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_magnetic_stripe_reading_devices_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "MSR Device Read-Only".to_string(),
            0x02..=0x10 => "Reserved".to_string(),
            0x11 => "Track 1 Length".to_string(),
            0x12 => "Track 2 Length".to_string(),
            0x13 => "Track 3 Length".to_string(),
            0x14 => "Track JIS Length".to_string(),
            0x15..=0x1F => "Reserved".to_string(),
            0x20 => "Track Data".to_string(),
            0x21 => "Track 1 Data".to_string(),
            0x22 => "Track 2 Data".to_string(),
            0x23 => "Track 3 Data".to_string(),
            0x24 => "Track JIS Data".to_string(),
            0x25..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_point_of_sale_usage_name(usage: u32) -> String {
        match usage {
            // TODO: Fix "USB HID Usage Tables"
            0x00..=0x8B => "See: USB HID Usage Tables".to_string(),
            0x8C => "Bar Code Scanner page".to_string(),
            0x8D => "Weighing Devices page".to_string(),
            0x8E => "Magnetic Stripe Reader page".to_string(),
            0x8F => "Reserved Point of Sale page".to_string(),
            0x90..=0xEFFF => "See: USB HID Usage Tables".to_string(),
            0xFF00..=0xFFFF => "Vendor-defined".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_camera_control_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01..=0x1F => "Reserved".to_string(),
            0x20 => "Camera Auto-focus".to_string(),
            0x21 => "Camera Shutter".to_string(),
            0x22..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_arcade_usage_name(usage: u32) -> String {
        match usage {
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_gaming_device_usage_name(usage: u32) -> String {
        match usage {
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_fido_alliance_usage_name(usage: u32) -> String {
        match usage {
            0x00 => "Undefined".to_string(),
            0x01 => "U2F Authenticator Device".to_string(),
            0x02..=0x1F => "Reserved".to_string(),
            0x20 => "Input Report Data".to_string(),
            0x21 => "Output Report Data".to_string(),
            0x22..=0xFFFF => "Reserved".to_string(),
            _ => format!("{:#04X}", usage),
        }
    }

    fn get_vendor_defined_usage_name(_usage_page: u32, usage: u32) -> String {
        // 0xFF00..=0xFFFF
        match usage {
            _ => format!("{:#04X}", usage),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::descriptor;

    #[test]
    fn telephony_works() {
        let bytes: [u8; 126] = [0x05, 0x0B, 0x09, 0x05, 0xA1, 0x01, 0x85, 0x02, 0x05, 0x0B, 0x15, 0x00, 0x25, 0x01, 0x09, 0x20, 0x09, 0x97, 0x09, 0x2A, 0x75, 0x01, 0x95, 0x03, 0x81, 0x23, 0x09, 0x2F, 0x09, 0x21, 0x09, 0x24, 0x09, 0x50, 0x75, 0x01, 0x95, 0x04, 0x81, 0x07, 0x09, 0x06, 0xA1, 0x02, 0x19, 0xB0, 0x29, 0xBB, 0x15, 0x00, 0x25, 0x0C, 0x75, 0x04, 0x95, 0x01, 0x81, 0x40, 0xC0, 0x09, 0x07, 0x15, 0x00, 0x25, 0x01, 0x05, 0x09, 0x75, 0x01, 0x95, 0x01, 0x81, 0x02, 0x75, 0x01, 0x95, 0x04, 0x81, 0x01, 0x05, 0x08, 0x15, 0x00, 0x25, 0x01, 0x09, 0x17, 0x09, 0x1E, 0x09, 0x09, 0x09, 0x18, 0x09, 0x20, 0x09, 0x21, 0x09, 0x2A, 0x75, 0x01, 0x95, 0x07, 0x91, 0x22, 0x05, 0x0B, 0x15, 0x00, 0x25, 0x01, 0x09, 0x9E, 0x75, 0x01, 0x95, 0x01, 0x91, 0x22, 0x75, 0x01, 0x95, 0x08, 0x91, 0x01, 0xC0];
        let report = descriptor::get_descriptor_report(&bytes);

        assert_eq!(report.items.len(), 64);

        assert!(matches!(&report.items[0].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[0].global_tag, Some(descriptor::HidGlobalTag::UsagePage(val)) if val == &(bytes[1] as u32)));

        assert!(matches!(&report.items[1].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[1].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[3] as u32)));

        assert!(matches!(&report.items[2].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[2].main_tag, Some(descriptor::HidMainTag::Collection(val)) if val == &descriptor::CollectionType::Application));

        assert!(matches!(&report.items[3].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[3].global_tag, Some(descriptor::HidGlobalTag::ReportId(val)) if val == &(bytes[7] as u32)));

        assert!(matches!(&report.items[4].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[4].global_tag, Some(descriptor::HidGlobalTag::UsagePage(val)) if val == &(bytes[9] as u32)));

        assert!(matches!(&report.items[5].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[5].global_tag, Some(descriptor::HidGlobalTag::LogicalMinimum(val)) if val == &(bytes[11] as i32)));

        assert!(matches!(&report.items[6].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[6].global_tag, Some(descriptor::HidGlobalTag::LogicalMaximum(val)) if val == &(bytes[13] as i32)));

        assert!(matches!(&report.items[7].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[7].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[15] as u32)));

        assert!(matches!(&report.items[8].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[8].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[17] as u32)));

        assert!(matches!(&report.items[9].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[9].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[19] as u32)));

        assert!(matches!(&report.items[10].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[10].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[21] as u32)));

        assert!(matches!(&report.items[11].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[11].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[23] as u32)));

        let main_data = descriptor::MainInputData {
            item_type: descriptor::ItemType::Constant,
            data_type: descriptor::DataType::Variable,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::NoPreferredState,
            null_state: descriptor::NullState::NoNullState,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[12].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[12].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Input(val) if val == &main_data));

        assert!(matches!(&report.items[13].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[13].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[27] as u32)));

        assert!(matches!(&report.items[14].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[14].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[29] as u32)));

        assert!(matches!(&report.items[15].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[15].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[31] as u32)));

        assert!(matches!(&report.items[16].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[16].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[33] as u32)));

        assert!(matches!(&report.items[17].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[17].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[35] as u32)));

        assert!(matches!(&report.items[18].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[18].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[37] as u32)));

        let main_data = descriptor::MainInputData {
            item_type: descriptor::ItemType::Constant,
            data_type: descriptor::DataType::Variable,
            data_point: descriptor::DataPoint::Relative,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::PreferredState,
            null_state: descriptor::NullState::NoNullState,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[19].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[19].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Input(val) if val == &main_data));

        assert!(matches!(&report.items[20].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[20].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[41] as u32)));

        assert!(matches!(&report.items[21].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[21].main_tag, Some(descriptor::HidMainTag::Collection(val)) if val == &descriptor::CollectionType::Logical));

        assert!(matches!(&report.items[22].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[22].local_tag, Some(descriptor::HidLocalTag::UsageMinimum(val)) if val == &(bytes[45] as u32)));

        assert!(matches!(&report.items[23].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[23].local_tag, Some(descriptor::HidLocalTag::UsageMaximum(val)) if val == &(bytes[47] as u32)));

        assert!(matches!(&report.items[24].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[24].global_tag, Some(descriptor::HidGlobalTag::LogicalMinimum(val)) if val == &(bytes[49] as i32)));

        assert!(matches!(&report.items[25].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[25].global_tag, Some(descriptor::HidGlobalTag::LogicalMaximum(val)) if val == &(bytes[51] as i32)));

        assert!(matches!(&report.items[26].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[26].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[53] as u32)));

        assert!(matches!(&report.items[27].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[27].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[55] as u32)));

        let main_data = descriptor::MainInputData {
            item_type: descriptor::ItemType::Data,
            data_type: descriptor::DataType::Array,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::PreferredState,
            null_state: descriptor::NullState::NullState,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[28].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[28].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Input(val) if val == &main_data));

        assert!(matches!(&report.items[29].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[29].main_tag, Some(descriptor::HidMainTag::EndCollection)));

        assert!(matches!(&report.items[30].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[30].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[60] as u32)));

        assert!(matches!(&report.items[31].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[31].global_tag, Some(descriptor::HidGlobalTag::LogicalMinimum(val)) if val == &(bytes[62] as i32)));

        assert!(matches!(&report.items[32].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[32].global_tag, Some(descriptor::HidGlobalTag::LogicalMaximum(val)) if val == &(bytes[64] as i32)));

        assert!(matches!(&report.items[33].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[33].global_tag, Some(descriptor::HidGlobalTag::UsagePage(val)) if val == &(bytes[66] as u32)));

        assert!(matches!(&report.items[34].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[34].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[68] as u32)));

        assert!(matches!(&report.items[35].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[35].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[70] as u32)));

        let main_data = descriptor::MainInputData {
            item_type: descriptor::ItemType::Data,
            data_type: descriptor::DataType::Variable,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::PreferredState,
            null_state: descriptor::NullState::NoNullState,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[36].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[36].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Input(val) if val == &main_data));

        assert!(matches!(&report.items[37].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[37].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[74] as u32)));

        assert!(matches!(&report.items[38].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[38].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[76] as u32)));

        let main_data = descriptor::MainInputData {
            item_type: descriptor::ItemType::Constant,
            data_type: descriptor::DataType::Array,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::PreferredState,
            null_state: descriptor::NullState::NoNullState,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[39].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[39].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Input(val) if val == &main_data));

        assert!(matches!(&report.items[40].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[40].global_tag, Some(descriptor::HidGlobalTag::UsagePage(val)) if val == &(bytes[80] as u32)));

        assert!(matches!(&report.items[41].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[41].global_tag, Some(descriptor::HidGlobalTag::LogicalMinimum(val)) if val == &(bytes[82] as i32)));

        assert!(matches!(&report.items[42].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[42].global_tag, Some(descriptor::HidGlobalTag::LogicalMaximum(val)) if val == &(bytes[84] as i32)));

        assert!(matches!(&report.items[43].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[43].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[86] as u32)));

        assert!(matches!(&report.items[44].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[44].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[88] as u32)));

        assert!(matches!(&report.items[45].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[45].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[90] as u32)));

        assert!(matches!(&report.items[46].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[46].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[92] as u32)));

        assert!(matches!(&report.items[47].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[47].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[94] as u32)));

        assert!(matches!(&report.items[48].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[48].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[96] as u32)));

        assert!(matches!(&report.items[49].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[49].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[98] as u32)));

        assert!(matches!(&report.items[50].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[50].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[100] as u32)));

        assert!(matches!(&report.items[51].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[51].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[102] as u32)));

        let main_data = descriptor::MainOutputData {
            item_type: descriptor::ItemType::Data,
            data_type: descriptor::DataType::Variable,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::NoPreferredState,
            null_state: descriptor::NullState::NoNullState,
            volatility: descriptor::Volatility::NonVolatile,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[52].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[52].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Output(val) if val == &main_data));

        assert!(matches!(&report.items[53].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[53].global_tag, Some(descriptor::HidGlobalTag::UsagePage(val)) if val == &(bytes[106] as u32)));

        assert!(matches!(&report.items[54].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[54].global_tag, Some(descriptor::HidGlobalTag::LogicalMinimum(val)) if val == &(bytes[108] as i32)));

        assert!(matches!(&report.items[55].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[55].global_tag, Some(descriptor::HidGlobalTag::LogicalMaximum(val)) if val == &(bytes[110] as i32)));

        assert!(matches!(&report.items[56].item_type, descriptor::HidItemType::Local));
        assert!(matches!(&report.items[56].local_tag, Some(descriptor::HidLocalTag::Usage(val)) if val == &(bytes[112] as u32)));

        assert!(matches!(&report.items[57].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[57].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[114] as u32)));

        assert!(matches!(&report.items[58].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[58].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[116] as u32)));

        let main_data = descriptor::MainOutputData {
            item_type: descriptor::ItemType::Data,
            data_type: descriptor::DataType::Variable,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::NoPreferredState,
            null_state: descriptor::NullState::NoNullState,
            volatility: descriptor::Volatility::NonVolatile,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[59].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[59].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Output(val) if val == &main_data));

        assert!(matches!(&report.items[60].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[60].global_tag, Some(descriptor::HidGlobalTag::ReportSize(val)) if val == &(bytes[120] as u32)));

        assert!(matches!(&report.items[61].item_type, descriptor::HidItemType::Global));
        assert!(matches!(&report.items[61].global_tag, Some(descriptor::HidGlobalTag::ReportCount(val)) if val == &(bytes[122] as u32)));

        let main_data = descriptor::MainOutputData {
            item_type: descriptor::ItemType::Constant,
            data_type: descriptor::DataType::Array,
            data_point: descriptor::DataPoint::Absolute,
            wrapping: descriptor::Wrapping::NoWrap,
            linearity: descriptor::Linearity::Linear,
            state_preferrence: descriptor::StatePreference::PreferredState,
            null_state: descriptor::NullState::NoNullState,
            volatility: descriptor::Volatility::NonVolatile,
            field_type: descriptor::FieldType::BitField,
        };
        assert!(matches!(&report.items[62].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[62].main_tag.as_ref().unwrap(), descriptor::HidMainTag::Output(val) if val == &main_data));

        assert!(matches!(&report.items[63].item_type, descriptor::HidItemType::Main));
        assert!(matches!(&report.items[63].main_tag, Some(descriptor::HidMainTag::EndCollection)));
    }
}