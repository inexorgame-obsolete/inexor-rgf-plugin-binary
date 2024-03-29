use crate::model::entity_model;
use crate::model::entity_ty;
use crate::Action;
use crate::BinaryData;
use crate::BinaryDataUrl;
use crate::File;
use crate::NAMESPACE_BINARY;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_SAVE_BINARY_DATA, NAMESPACE_BINARY, ENTITY_TYPE_NAME_SAVE_BINARY_DATA, "save_binary_data");

entity_model!(SaveBinaryData);
impl BinaryData for SaveBinaryData {}
impl BinaryDataUrl for SaveBinaryData {}
impl File for SaveBinaryData {}
impl Action for SaveBinaryData {}
// impl model_base::Named for SaveBinaryData {}
