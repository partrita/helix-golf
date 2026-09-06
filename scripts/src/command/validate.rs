//! Validate that all of the examples adhere to a certain structure

use std::{collections::HashSet, env, fmt::Write as _, fs};

use crate::{
    command::{GENERATED_DIR, ROOT_DIR},
    parse_example::Example,
};
use miette::miette;
use tap::Pipe as _;

/// Make sure each example has the required structure
pub fn validate() -> miette::Result<Vec<Example>> {
    // If user passes any examples, those will be the only ones that are included.
    //
    // If no examples are passed, then include everything
    let only_include_these_examples: HashSet<_> = env::args()
        // 1. skip binary name
        // 2. skip argument type
        .skip(2)
        .collect();

    fs::create_dir_all(&*GENERATED_DIR)
        .and_then(|()| fs::remove_dir_all(&*GENERATED_DIR))
        .and_then(|()| fs::create_dir_all(&*GENERATED_DIR))
        .map_err(|err| miette!("failed cleaning the generated directory: {err}"))?;

    let mut examples = Example::parse_all(&ROOT_DIR, &only_include_these_examples)?;

    // We want to sort examples from smallest command count to largest
    examples.sort_by_key(|a| a.key_events.len());

    examples
        .iter()
        .try_fold(
            (
                String::new(),
                String::from(
                    "<!-- @generated This file is generated. Do not edit it by hand. -->

# Summary

- [Helix Golf - Introduction](introduction.md)\n",
                ),
                String::new(),
            ),
            |(mut all_previews, mut summary_md, mut md_file_with_everything),
             example|
             -> miette::Result<(String, String, String)> {
                let name = &example.name;
                let title = &example.title;

                writeln!(&mut summary_md, "- [{title}]({name}.md)").map_err(|err| {
                    miette!("failed to add line to SUMMARY.md for example `{name}`: {err}",)
                })?;

                writeln!(&mut md_file_with_everything, "{}", example.contents).map_err(|err| {
                    miette!(
                        "failed to add entire example to \
                        introduction.md for example `{name}`: {err}",
                    )
                })?;

                writeln!(
                    &mut all_previews,
                    "## [{title}]({name}.md)

{desc}

<video autoplay controls loop>
  <source src=\"generated/{name}.mp4\">
</video>\n\n",
                    desc = example.description.as_deref().unwrap_or("")
                )
                .map_err(|err| {
                    miette!("failed to add line to SUMMARY.md for example `{name}`: {err}",)
                })?;

                Ok((all_previews, summary_md, md_file_with_everything))
            },
        )?
        .pipe(|(all_previews, summary_md, md_file_with_everything)| {
            fs::write(ROOT_DIR.join("SUMMARY.md"), summary_md)
                .map_err(|err| miette!("Failed to write `SUMMARY.md`: {err}"))
                .map(|()| {
                    fs::write(
                        ROOT_DIR.join("introduction.md"),
                        format!(
                            "<!-- @generated This file is generated. Do not edit it by hand. -->

# Helix Golf

Helix Golf는 Rust로 작성된 차세대 터미널 IDE인
[Helix 에디터](https://github.com/helix-editor/helix)를 사용한 리팩토링 예제 모음입니다.

각 예제는 상세히 설명되어 있으며 최신 버전의 Helix로 테스트되었고 비디오 데모가 포함되어 있습니다.
예제들은 단순한 가상이 아니며, 모두 실제 상황에서 만들어졌습니다.

다중 커서가 Helix의 핵심 편집 기본 요소이기 때문에, 많은 경우 Helix Golf 예제는
유사한 Vim Golf 예제보다 훨씬 이해하기 쉽고 _직접 떠올리기 쉬우며_ 종종 더 짧습니다.

이는 Helix를 개발자와 텍스트 편집 속도를 높이고자 하는 모든 사람을 위한
완벽한 맥가이버 칼 같은 텍스트 에디터로 만들어 줍니다.
생산성이 향상될 뿐만 아니라 정말 재미있습니다!

# 각 예제 데모

<details>

<summary>모든 예제는 하나의 코드 블록으로도 제공됩니다</summary>

인터넷 없이도 Helix에 복사하여 붙여넣고 직접 연습해 볼 수 있습니다!

````````````md
{md_file_with_everything}
````````````

</details>

{all_previews}"
                        ),
                    )
                })
        })?
        .map_err(|err| miette!("Failed to write `introduction.md`: {err}"))?;

    Ok(examples)
}
