#[cfg(test)]
mod tests {
    use icon_derive::tabler_icon;

    #[test]
    fn init() {
        tabler_icon! {
            // -- global scoup
            // ----------------------
            #[name = "super_user"] // -- outer
            user[outline, filled],  // --
            #[name = "super_telegram"] // -- outer
            brand_telegram[outline], // --
            #[name="Crates"] // -- outer
            box_[outline], // --
        }

        // impl std::str::FromStr for Tabler {
        //     type Err = String;

        //     fn from_str(s: &str) -> Result<Self, Self::Err> {
        //         let normalized = s.trim().to_lowercase();
        //         match normalized.as_str() {
        //             "nn" => Ok(Self::FILLED_USER),
        //             _ => Err(format!("Unknown icon variant: {}", normalized)),
        //         }
        //     }
        // }
    }
}
