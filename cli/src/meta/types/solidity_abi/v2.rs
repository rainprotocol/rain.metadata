use serde::Serialize;
use serde::de::Error;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use validator::Validate;
use schemars::JsonSchema;
use alloy_json_abi::JsonAbi;
use validator::ValidationErrors;
use serde::ser::SerializeStruct;
use super::super::super::MetaMap;


/// # SolidityABI
/// JSON representation of a Solidity ABI interface. can be switched to ethers ABI struct using TryFrom trait
/// https://docs.soliditylang.org/en/latest/abi-spec.html#json
#[derive(JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SolidityAbiMeta(Vec<SolidityAbiItem>);

impl Validate for SolidityAbiMeta {
    fn validate(&self) -> Result<(), ValidationErrors> {
        ValidationErrors::merge_all(
            Ok(()),
            "root",
            self.0.iter().map(|item| item.validate()).collect()
        )
    }
}

impl TryFrom<Vec<u8>> for SolidityAbiMeta {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match serde_json::from_slice::<Self>(&value).map_err(anyhow::Error::from) {
            Ok(t) => match t.validate().map_err(anyhow::Error::from) {
                Ok(()) => Ok(t),
                Err(e) => Err(e),
            },
            Err(e) => Err(e)
        }
    }
}

impl TryFrom<MetaMap> for SolidityAbiMeta {
    type Error = anyhow::Error;
    fn try_from(value: MetaMap) -> Result<Self, Self::Error> {
        Self::try_from(value.unpack()?)
    }
}

impl TryFrom<MetaMap> for JsonAbi {
    type Error = anyhow::Error;
    fn try_from(value: MetaMap) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(value.unpack()?.as_slice())?)
    }
}

impl TryFrom<SolidityAbiMeta> for JsonAbi {
    type Error = anyhow::Error;
    fn try_from(value: SolidityAbiMeta) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(serde_json::to_string(&value)?.as_str())?)
    }
}

impl TryFrom<JsonAbi> for SolidityAbiMeta {
    type Error = anyhow::Error;
    fn try_from(value: JsonAbi) -> Result<Self, Self::Error> {
        Ok(serde_json::from_value(serde_json::to_value(value)?)?)
    }
}

#[derive(Validate, JsonSchema, Debug, Clone, PartialEq)]
pub struct SolidityAbiItemFn {
    inputs: Vec<SolidityAbiFnIO>,
    name: String,
    outputs: Vec<SolidityAbiFnIO>,
    state_mutability: SolidityAbiFnMutability,
}

impl Serialize for SolidityAbiItemFn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SolidityAbiItemFn", 5)?;
        state.serialize_field("inputs", &self.inputs)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("outputs", &self.outputs)?;
        state.serialize_field("stateMutability", &self.state_mutability)?;
        state.serialize_field("type", "function")?;
        state.end()
    }
}

#[derive(Validate, JsonSchema, Debug, Clone, PartialEq)]
pub struct SolidityAbiItemConstructor {
    inputs: Vec<SolidityAbiFnIO>,
    state_mutability: SolidityAbiFnMutability,
}

impl Serialize for SolidityAbiItemConstructor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SolidityAbiItemConstructor", 3)?;
        state.serialize_field("inputs", &self.inputs)?;
        state.serialize_field("stateMutability", &self.state_mutability)?;
        state.serialize_field("type", "constructor")?;
        state.end()
    }
}

#[derive(Validate, JsonSchema, Debug, Clone, PartialEq)]
pub struct SolidityAbiItemReceive {
    state_mutability: SolidityAbiFnMutability,
}

impl Serialize for SolidityAbiItemReceive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SolidityAbiItemReceive", 2)?;
        state.serialize_field("stateMutability", &self.state_mutability)?;
        state.serialize_field("type", "receive")?;
        state.end()
    }
}

#[derive(Validate, JsonSchema, Debug, Clone, PartialEq)]
pub struct SolidityAbiItemFallback {
    state_mutability: SolidityAbiFnMutability,
}

impl Serialize for SolidityAbiItemFallback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SolidityAbiItemFallback", 2)?;
        state.serialize_field("stateMutability", &self.state_mutability)?;
        state.serialize_field("type", "fallback")?;
        state.end()
    }
}

#[derive(Validate, JsonSchema, Debug, Clone, PartialEq)]
pub struct SolidityAbiItemEvent {
    anonymous: bool,
    inputs: Vec<SolidityAbiEventInput>,
    name: String,
}

impl Serialize for SolidityAbiItemEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SolidityAbiItemEvent", 4)?;
        state.serialize_field("anonymous", &self.anonymous)?;
        state.serialize_field("inputs", &self.inputs)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("type", "event")?;
        state.end()
    }
}

#[derive(Validate, JsonSchema, Debug, Clone, PartialEq)]
pub struct SolidityAbiItemError {
    inputs: Vec<SolidityAbiErrorInput>,
    name: String,
}

impl Serialize for SolidityAbiItemError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SolidityAbiItemError", 3)?;
        state.serialize_field("inputs", &self.inputs)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("type", "error")?;
        state.end()
    }
}

#[derive(JsonSchema, Debug, Clone, PartialEq)]
pub enum SolidityAbiItem {
    Function(SolidityAbiItemFn),
    Constructor(SolidityAbiItemConstructor),
    Receive(SolidityAbiItemReceive),
    Fallback(SolidityAbiItemFallback),
    Event(SolidityAbiItemEvent),
    Error(SolidityAbiItemError),
}

impl Serialize for SolidityAbiItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SolidityAbiItem::Function(item_fn) => item_fn.serialize(serializer),
            SolidityAbiItem::Constructor(item_constructor) => item_constructor.serialize(serializer),
            SolidityAbiItem::Receive(item_receive) => item_receive.serialize(serializer),
            SolidityAbiItem::Fallback(item_fallback) => item_fallback.serialize(serializer),
            SolidityAbiItem::Event(item_event) => item_event.serialize(serializer),
            SolidityAbiItem::Error(item_error) => item_error.serialize(serializer),
        }
    }
}

impl Validate for SolidityAbiItem {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            SolidityAbiItem::Function(item_fn) => item_fn.validate(),
            SolidityAbiItem::Constructor(item_constructor) => item_constructor.validate(),
            SolidityAbiItem::Receive(item_receive) => item_receive.validate(),
            SolidityAbiItem::Fallback(item_fallback) => item_fallback.validate(),
            SolidityAbiItem::Event(item_event) => item_event.validate(),
            SolidityAbiItem::Error(item_error) => item_error.validate(),
        }
    }
}

#[derive(JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SolidityAbiFnMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiFnIO {
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<SolidityAbiFnIO>>,
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiErrorInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<SolidityAbiErrorInput>>,
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiEventInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<SolidityAbiEventInputComponent>>,
    indexed: bool,
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiEventInputComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<SolidityAbiEventInputComponent>>,
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    typ: String,
}

impl<'de> Deserialize<'de> for SolidityAbiItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Intermediate {
            #[serde(rename = "type")]
            typ: IntermediateType,
            name: Option<String>,
            inputs: Option<Vec<IntermediateIO>>,
            outputs: Option<Vec<IntermediateIO>>,
            state_mutability: Option<SolidityAbiFnMutability>,
            anonymous: Option<bool>
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum IntermediateType {
            Function,
            Constructor,
            Receive,
            Fallback,
            Event,
            Error,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct IntermediateIO {
            internal_type: String,
            name: String,
            #[serde(rename = "type")]
            typ: String,
            components: Option<Vec<IntermediateIO>>,
            indexed: Option<bool>,
        }

        let intermediate = Intermediate::deserialize(deserializer)?;

        fn map_item_fn_io(intermediate_io: &IntermediateIO) -> Result<SolidityAbiFnIO, String>{
            if intermediate_io.indexed.is_some() {
                return Err("indexed found on fn io".into());
            }

            let components: Option<Vec<SolidityAbiFnIO>> = match &intermediate_io.components {
                Some(cs) => {
                    let result: Result<Vec<SolidityAbiFnIO>, String> = cs.iter().map(map_item_fn_io).collect();
                    Some(result?)
                },
                None => None,
            };
            Ok(SolidityAbiFnIO {
                name: intermediate_io.name.clone(),
                typ: intermediate_io.typ.clone(),
                internal_type: intermediate_io.internal_type.clone(),
                components,
            })
        }

        fn map_item_event_input(intermediate_io: &IntermediateIO) -> Result<SolidityAbiEventInput, String> {
            fn map_item_event_input_component(intermediate_io: &IntermediateIO) -> Result<SolidityAbiEventInputComponent, String> {
                if intermediate_io.indexed.is_some() {
                    return Err("indexed found on event component".into());
                }

                let components: Option<Vec<SolidityAbiEventInputComponent>> = match &intermediate_io.components {
                    Some(cs) => {
                        let result: Result<Vec<SolidityAbiEventInputComponent>, String> = cs.iter().map(map_item_event_input_component).collect();
                        Some(result?)
                    },
                    None => None,
                };
                Ok(SolidityAbiEventInputComponent {
                    components,
                    internal_type: intermediate_io.internal_type.clone(),
                    name: intermediate_io.name.clone(),
                    typ: intermediate_io.typ.clone(),
                })
            }

            let components: Option<Vec<SolidityAbiEventInputComponent>> = match &intermediate_io.components {
                Some(cs) => {
                    let result: Result<Vec<SolidityAbiEventInputComponent>, String> = cs.iter().map(map_item_event_input_component).collect();
                    Some(result?)
                },
                None => None,
            };

            Ok(SolidityAbiEventInput {
                components,
                indexed: intermediate_io.indexed.ok_or::<String>("indexed missing on event input".into())?,
                internal_type: intermediate_io.internal_type.clone(),
                name: intermediate_io.name.clone(),
                typ: intermediate_io.typ.clone(),
            })
        }

        fn map_item_error_input(intermediate_io: &IntermediateIO) -> Result<SolidityAbiErrorInput, String> {
            if intermediate_io.indexed.is_some() {
                return Err("indexed found on fn io".into());
            }

            let components: Option<Vec<SolidityAbiErrorInput>> = match &intermediate_io.components {
                Some(cs) => {
                    let result: Result<Vec<SolidityAbiErrorInput>, String> = cs.iter().map(map_item_error_input).collect();
                    Some(result?)
                },
                None => None,
            };
            Ok(SolidityAbiErrorInput {
                components,
                internal_type: intermediate_io.internal_type.clone(),
                name: intermediate_io.name.clone(),
                typ: intermediate_io.typ.clone(),
            })
        }

        match intermediate.typ {
            IntermediateType::Function => {
                let inputs: Vec<SolidityAbiFnIO> = match intermediate.inputs {
                    Some(is) => {
                        let result: Result<Vec<SolidityAbiFnIO>, String> = is.iter().map(map_item_fn_io).collect();
                        result.map_err(|e| D::Error::custom(e))?
                    },
                    None => vec![],
                };
                let outputs: Vec<SolidityAbiFnIO> = match intermediate.outputs {
                    Some(os) => {
                        let result: Result<Vec<SolidityAbiFnIO>, String> = os.iter().map(map_item_fn_io).collect();
                        result.map_err(|e| D::Error::custom(e))?
                    },
                    None => vec![],
                };
                Ok(SolidityAbiItem::Function(SolidityAbiItemFn {
                    name: intermediate.name.ok_or(D::Error::custom("function missing name"))?,
                    inputs,
                    outputs,
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("function missing mutability"))?,
                }))
            },
            IntermediateType::Constructor => {
                let inputs: Vec<SolidityAbiFnIO> = match intermediate.inputs {
                    Some(is) => {
                        let result: Result<Vec<SolidityAbiFnIO>, String> = is.iter().map(map_item_fn_io).collect();
                        result.map_err(|e| D::Error::custom(e))?
                    },
                    None => vec![],
                };
                Ok(SolidityAbiItem::Constructor(SolidityAbiItemConstructor {
                    inputs,
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("constructor missing mutability"))?,
                }))
            },
            IntermediateType::Receive => {
                Ok(SolidityAbiItem::Receive(SolidityAbiItemReceive {
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("receive missing mutability"))?,
                }))
            },
            IntermediateType::Fallback => {
                Ok(SolidityAbiItem::Fallback(SolidityAbiItemFallback {
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("fallback missing mutability"))?,
                }))
            },
            IntermediateType::Event => {
                let inputs: Vec<SolidityAbiEventInput> = match intermediate.inputs {
                    Some(is) => {
                        let result: Result<Vec<SolidityAbiEventInput>, String> = is.iter().map(map_item_event_input).collect();
                        result.map_err(|e| D::Error::custom(e))?
                    },
                    None => vec![],
                };
                Ok(SolidityAbiItem::Event(SolidityAbiItemEvent {
                    name: intermediate.name.ok_or(D::Error::custom("event missing name"))?,
                    inputs,
                    anonymous: intermediate.anonymous.ok_or(D::Error::custom("event missing anonymous"))?,
                }))
            },
            IntermediateType::Error => {
                let inputs: Vec<SolidityAbiErrorInput> = match intermediate.inputs {
                    Some(is) => {
                        let result: Result<Vec<SolidityAbiErrorInput>, String> = is.iter().map(map_item_error_input).collect();
                        result.map_err(|e| D::Error::custom(e))?
                    },
                    None => vec![],
                };
                Ok(SolidityAbiItem::Error(SolidityAbiItemError {
                    name: intermediate.name.ok_or(D::Error::custom("error missing name"))?,
                    inputs,
                }))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use alloy_json_abi::JsonAbi;
    use super::SolidityAbiMeta;

    // test json roundtrip for SolidityAbiMeta and alloy JsonAbi
    #[test]
    fn test_json_roundtrip() -> anyhow::Result<()> {
        let path = "./test/abis";
        for file in std::fs::read_dir(path)? {
            let file = file?;
            let original_json_value: serde_json::Value = serde_json::from_slice(std::fs::read(file.path())?.as_slice())?;
            let original_json_abi : serde_json::Value = original_json_value["abi"].clone();

            let solidity_abi_meta: SolidityAbiMeta = serde_json::from_value(original_json_abi.clone())?;
            assert_eq!(original_json_abi, serde_json::to_value(&solidity_abi_meta)?);

            // since alloy JsonAbi doesn't keep the original order of abi items, we need to check item by item
            let json_abi_alloy: JsonAbi = serde_json::from_str(original_json_abi.clone().to_string().as_str())?;
            for e in original_json_abi.as_array().unwrap().iter() {
                if let None = json_abi_alloy.items().find(|item| &serde_json::to_value(item).unwrap() == e) {
                    return Err(anyhow::anyhow!("roundtrip failed!"));
                }
            }
        }

        Ok(())
    }

    // test conversion between SolidityAbiMeta and alloy JsonAbi
    #[test]
    fn test_abi_conversion() -> anyhow::Result<()> {
        let path = "./test/abis";
        for file in std::fs::read_dir(path)? {
            let file = file?;
            let original_json_value: serde_json::Value = serde_json::from_slice(std::fs::read(file.path())?.as_slice())?;
            let original_json_abi : serde_json::Value = original_json_value["abi"].clone();

            let solidity_abi_meta: SolidityAbiMeta = serde_json::from_value(original_json_abi.clone())?;
            let json_abi_alloy: JsonAbi = serde_json::from_str(original_json_abi.clone().to_string().as_str())?;

            let converted_json_abi: JsonAbi = solidity_abi_meta.clone().try_into()?;
            assert_eq!(converted_json_abi, json_abi_alloy);

            // since alloy JsonAbi doesn't keep the original order of abi items, we need to check item by item
            let converted_abi_meta: SolidityAbiMeta = json_abi_alloy.clone().try_into()?;
            for item in solidity_abi_meta.0.iter() {
                if let Some(v) = converted_abi_meta.0.iter().find(|e| *e == item ) {
                    assert_eq!(v, item);
                } else {
                    return Err(anyhow::anyhow!("wrong conversion!"))
                };
            }
        }

        Ok(())
    }
}