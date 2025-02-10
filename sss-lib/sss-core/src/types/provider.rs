use serde::{Deserialize, Serialize};
use tabler_icon_definer::tabler_icon;

tabler_icon! {
    #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
    #[derive(Serialize, Deserialize)]
    #[name="github"]
    brand_github[outline, filled],
    #[name="telegram"]
    brand_telegram[outline],
    #[name="Crates"]
    box_[outline],
    #[name="linkedin"]
    brand_linkedin[outline, filled],
    #[name="discord"]
    brand_discord[outline, filled],
    #[name="reddit"]
    brand_reddit[outline],
}
#[cfg(test)]
mod tests {

    use crate::types::provider::Tabler;

    #[test]
    fn check_svg2() {
        println!("{}", Tabler::FILLED_GITHUB);
        println!("{}", Tabler::FILLED_GITHUB.as_str_merget())
    }
}
