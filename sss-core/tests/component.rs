// #[cfg(test)]
// mod component {

//     use std::fmt::Write;

//     use super::*;

//     #[test]
//     fn create_component() {
//         #[derive(Debug, Default, Clone)]
//         struct Component {
//             childrens: Vec<Self>,
//             tag: String,
//             class: String,
//             style: String,
//             self_close: bool,
//         }
//         impl ComponentBuilder for Component {
//             fn tag(self, tag: String) -> Self {
//                 todo!()
//             }

//             fn attributes(
//                 self,
//                 attributes: Option<std::collections::HashMap<String, String>>,
//             ) -> Self {
//                 if let Some(item) = attributes.as_ref() {
//                     let mut class = String::new();
//                     class.push_str("class: ");
//                     let mut style = String::new();
//                     class.push_str("style: ");
//                     let mut another = String::new();

//                     for (key, item) in item {
//                         match key.as_str() {
//                             "class" => {
//                                 class.push_str(item);
//                                 class.push(',');
//                                 class.push(' ');
//                             }
//                             "style" => {
//                                 style.push_str(item);
//                                 style.push(',');
//                                 style.push(' ');
//                             }
//                             another => {
//                                 class.push_str("style: ");
//                                 style.push_str(item);
//                                 style.push(',');
//                                 style.push(' ');
//                             }
//                         }
//                     }
//                 }

//                 self
//             }

//             fn self_close(self, close: bool) -> Self {
//                 todo!()
//             }

//             #[allow(unconditional_recursion)]
//             fn child(self, childrens: Vec<Self>) -> Self {
//                 self.child(childrens)
//             }
//         }
//     }
// }
