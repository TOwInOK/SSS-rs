use sss_core::prelude::{About, Blank, Skill, Skills, SocialMedias, Specifications, User};
pub mod icon;

#[derive(Debug, Default)]
pub struct Sections {
    user_info: User,
    specifications: Specifications,
    about: About,
    socials: SocialMedias,
    skills: Skills,
}

impl Sections {
    pub fn user_info(&self) -> &User {
        &self.user_info
    }

    pub fn specifications(&self) -> &[String] {
        &self.specifications
    }

    pub fn about(&self) -> &About {
        &self.about
    }

    pub fn socials(&self) -> &[Blank] {
        &self.socials
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn set_user_info(
        &mut self,
        user_info: User,
    ) {
        self.user_info = user_info;
    }

    pub fn set_specifications(
        &mut self,
        specifications: Specifications,
    ) {
        self.specifications = specifications;
    }

    pub fn set_about(
        &mut self,
        about: About,
    ) {
        self.about = about;
    }

    pub fn set_socials(
        &mut self,
        socials: SocialMedias,
    ) {
        self.socials = socials;
    }

    pub fn set_skills(
        &mut self,
        skills: Skills,
    ) {
        self.skills = skills;
    }

    pub fn user_info_mut(&mut self) -> &mut User {
        &mut self.user_info
    }

    pub fn specifications_mut(&mut self) -> &mut Specifications {
        &mut self.specifications
    }

    pub fn about_mut(&mut self) -> &mut About {
        &mut self.about
    }

    pub fn socials_mut(&mut self) -> &mut SocialMedias {
        &mut self.socials
    }

    pub fn skills_mut(&mut self) -> &mut Skills {
        &mut self.skills
    }
}

pub fn sections() -> Sections {
    Sections::default()
}
