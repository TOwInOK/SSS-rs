#[cfg(test)]
mod tests {
    use tabler_icon_definer::tabler_icon;

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
    }
}
