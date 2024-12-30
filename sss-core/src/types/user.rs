use serde::{Deserialize, Serialize};

pub type Skills = Vec<Skill>;
pub type Nicknames = Vec<Nickname>;
pub type Specifications = Vec<String>;
pub type SocialMedias = Vec<Blank>;
pub type TopProjects = Vec<Blank>;

pub fn user() -> User {
    User::default()
}

pub fn nickname() -> Nickname {
    Nickname::default()
}

pub fn about() -> About {
    About::default()
}

pub fn blank() -> Blank {
    Blank::default()
}

pub fn skill() -> Skill {
    Skill::default()
}

pub fn since() -> Since {
    Since::default()
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub name: String,
    pub nickname: Nickname,
    pub other_nicknames: Nicknames,
}

impl User {
    pub fn name(
        mut self,
        name: String,
    ) -> Self {
        self.name = name;
        self
    }

    pub fn nickname(
        mut self,
        nickname: Nickname,
    ) -> Self {
        self.nickname = nickname;
        self
    }

    pub fn other_nicknames(
        mut self,
        other_nicknames: Nicknames,
    ) -> Self {
        self.other_nicknames = other_nicknames;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Nickname {
    pub word: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronounce: Option<String>,
}

impl Nickname {
    pub fn word(
        mut self,
        word: String,
    ) -> Self {
        self.word = word;
        self
    }

    pub fn pronounce(
        mut self,
        pronounce: Option<String>,
    ) -> Self {
        self.pronounce = pronounce;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct About {
    pub text: String,
    #[serde(default = "default_max_length")]
    pub max_length: usize,
}

impl About {
    pub fn text(
        mut self,
        text: String,
    ) -> Self {
        self.text = text;
        self
    }

    pub fn max_length(
        mut self,
        max_length: usize,
    ) -> Self {
        self.max_length = max_length;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Blank {
    pub provider: String,
    pub link: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default)]
    pub main: bool,
}

impl Blank {
    pub fn provider(
        mut self,
        provider: String,
    ) -> Self {
        self.provider = provider;
        self
    }

    pub fn link(
        mut self,
        link: String,
    ) -> Self {
        self.link = link;
        self
    }

    pub fn logo(
        mut self,
        logo: Option<String>,
    ) -> Self {
        self.logo = logo;
        self
    }

    pub fn main(
        mut self,
        main: bool,
    ) -> Self {
        self.main = main;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Skill {
    pub skill: String,
    pub top_projects: TopProjects,
    #[serde(default)]
    pub since: Option<Since>,
    #[serde(default)]
    pub main: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainer_site: Option<String>,
}

impl Skill {
    pub fn skill(
        mut self,
        skill: String,
    ) -> Self {
        self.skill = skill;
        self
    }

    pub fn top_projects(
        mut self,
        top_projects: TopProjects,
    ) -> Self {
        self.top_projects = top_projects;
        self
    }

    pub fn since(
        mut self,
        since: Option<Since>,
    ) -> Self {
        self.since = since;
        self
    }

    pub fn main(
        mut self,
        main: bool,
    ) -> Self {
        self.main = main;
        self
    }

    pub fn maintainer_site(
        mut self,
        maintainer_site: Option<String>,
    ) -> Self {
        self.maintainer_site = maintainer_site;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Since {
    pub start: usize,
    #[serde(default)]
    pub end: usize,
}

impl Since {
    pub fn start(
        mut self,
        start: usize,
    ) -> Self {
        self.start = start;
        self
    }

    pub fn end(
        mut self,
        end: usize,
    ) -> Self {
        self.end = end;
        self
    }
}

impl std::fmt::Display for Since {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.end == 0 {
            write!(f, "{} -> today", self.start)
        } else {
            write!(f, "{} -> {}", self.start, self.end)
        }
    }
}

fn default_max_length() -> usize {
    100
}
