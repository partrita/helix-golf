# Replace a Regex-Sensitive Literal

<!-- difficulty: intermediate -->

문서 전체에서 정규식 메타문자가 포함된 문자열을 다른 문자열로 한 번에 변경합니다.

## Before

```text
C++ is widely used.
I learned C++ before Rust.
This project does not use C++ anymore.
```

## After

```text
Rust is widely used.
I learned Rust before Rust.
This project does not use Rust anymore.
```

## Command

```
%sC\+\+<ret>cRust<esc>
```

1. `%` 문서 전체를 하나의 선택 영역으로 만듭니다.
1. `s` 정규식 선택 명령을 시작합니다.
1. `C\+\+` 메타문자 플러스를 이스케이프하여 문자 그대로의 C++만 찾습니다.
1. `<ret>` 정규식을 확정하고 문서 전체의 모든 C++ 일치 항목을 각각의 선택 영역으로 만듭니다.
1. `c` 선택 영역의 내용을 삭제하고 입력 모드로 진입합니다.
1. `Rust` 변경할 텍스트를 입력합니다.
1. `<esc>` 일반 모드로 돌아갑니다.
