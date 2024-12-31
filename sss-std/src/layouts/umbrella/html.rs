use components::prelude::*;
use render::prelude::*;
use sss_core::prelude::*;
// TODO: Change to askama render
pub struct UmbrellaHtmlRender;
impl Layout<String, Sections, Theme> for UmbrellaHtmlRender {
    fn render(
        component: Sections,
        shade: &Theme,
    ) -> String {
        let user_info = |user: &User| {
            format!(
                r#"<div class="flex justify-evenly items-center gap-4 text-[{}]">
                           <div class="text-lg md:text-xl font-bold font-mono">
                               Some({})
                           </div>
                           <div class="flex items-center">
                               <span class="text-2xl md:text-3xl font-mono">[</span>
                               <div class="flex flex-col items-center mx-2">
                                   <span class="text-xs md:text-sm font-mono">{}</span>
                                   <span class="text-[10px] md:text-xs text-[{}]/80 font-mono tracking-widest">
                                       {}
                                   </span>
                               </div>
                               <span class="text-2xl md:text-3xl font-mono">]</span>
                           </div>
                       </div>"#,
                shade.get_colors().primary,
                user.nickname.word,
                user.nickname.word,
                shade.get_colors().thirdly,
                user.nickname
                    .pronounce
                    .as_ref()
                    .map_or(String::new(), |s| s.to_string())
            )
        };

        let specs = |specs: &[String]| {
            format!(
                r#"<div class="grid grid-cols-2 gap-2 md:gap-4">
                    <div class="grid gap-2">
                        {}
                    </div>
                    <div class="text-[{}] text-right font-mono text-sm md:text-base max-h-[{}px]">
                        {}
                    </div>
                </div>"#,
                specs
                    .iter()
                    .map(|spec| format!(
                        r#"<div class="text-[{}] flex items-center max-w-[120px]">
                            <span class="text-xl md:text-2xl font-mono w-[15px]">[</span>
                            <span class="text-xs md:text-sm font-mono text-center flex-1">{}</span>
                            <span class="text-xl md:text-2xl font-mono w-[15px]">]</span>
                        </div>"#,
                        shade.get_colors().primary,
                        spec
                    ))
                    .collect::<Vec<_>>()
                    .join(""),
                shade.get_colors().thirdly,
                component.about().max_length,
                component.about().text
            )
        };

        let socials = |socials: &[Blank]| {
            format!(
                r#"<div class="grid grid-cols-3 gap-8 md:gap-20 text-[{}]">
                    {}
                </div>"#,
                shade.get_colors().primary,
                socials
                    .iter()
                    .map(|social| format!(
                        r#"<div class="flex items-center justify-center">
                            <span class="text-xl md:text-2xl font-mono">[</span>
                            <div class="h-[48px] flex-col justify-evenly items-center inline-flex">
                                <svg xmlns="http://www.w3.org/2000/svg" class="text-[{}] size-12 md:size-20" {}>
                                </svg>
                                <span class="text-xs md:text-sm font-mono mx-2">{}</span>
                            </div>
                            <span class="text-xl md:text-2xl font-mono">]</span>
                        </div>"#,
                        shade.get_colors().thirdly,
                        social.logo.as_ref().map_or(String::new(), |s| s.to_string()),
                        social.provider
                    ))
                    .collect::<Vec<_>>()
                    .join("")
            )
        };

        let skills = |skills: &[Skill]| {
            format!(
                r#"<div class="flex justify-evenly gap-4">
                    {}
                </div>"#,
                skills
                    .iter()
                    .map(|skill| format!(
                        r#"<div class="text-center">
                            <div class="text-xl md:text-2xl text-[{}] font-mono">
                                {}
                            </div>
                            <div class="text-xs md:text-sm text-[{}]/80 font-mono">
                                {}
                            </div>
                            <div class="text-xs md:text-sm text-[{}]/80 font-mono">
                                {}
                            </div>
                        </div>"#,
                        shade.get_colors().primary,
                        skill.skill,
                        shade.get_colors().thirdly,
                        skill
                            .since
                            .as_ref()
                            .map_or("".to_string(), |s| s.to_string()),
                        shade.get_colors().border,
                        skill.maintainer_site.as_deref().unwrap_or("")
                    ))
                    .collect::<Vec<_>>()
                    .join("")
            )
        };

        format!(
            r#"<div class="p-4 rounded-lg border-t-2 border-b-2 border-[{}] flex flex-col gap-4">
                     {}
                     {}
                     {}
                     {}
                 </div>"#,
            shade.get_colors().thirdly,
            user_info(component.user_info()),
            specs(component.specifications()),
            socials(component.socials()),
            skills(component.skills())
        )
    }

    fn finylize(
        rendered: String,
        shade: &Theme,
    ) -> String {
        format!(
            r#"<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <script src="https://cdn.tailwindcss.com"></script>
                        <script>
                            tailwind.config = {{
                                theme: {{
                                    extend: {{
                                        fontFamily: {{
                                            mono: ['PT Mono', 'monospace'],
                                        }},
                                    }},
                                }},
                            }};
                        </script>
                        <link rel="preconnect" href="https://fonts.googleapis.com">
                        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                        <link href="https://fonts.googleapis.com/css2?family=PT+Mono&display=swap" rel="stylesheet">
                    </head>
                    <body class="min-h-screen bg-[{}] flex justify-center items-center p-4">
                        <div class="max-w-md w-full bg-black/20 rounded-2xl border border-black p-1">
                            {}
                        </div>
                    </body>
                </html>"#,
            shade.get_colors().secondary,
            rendered
        )
    }
}
