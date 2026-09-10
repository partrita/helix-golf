# Delete Surround

여러 줄의 감싸는 괄호를 한 번에 제거합니다.

<!-- difficulty: intermediate -->

## Before

```text
(use)
(use)
```

## After

```text
use
use
```

## Command

```
%suse<ret>md(
```

1. `%` 전체 파일 선택
1. `s` 정규식으로 일치 항목 선택
1. `use` 괄호 안의 텍스트 입력
1. `<ret>` 정규식 확정하고 두 단어를 각각 선택
1. `md(` 각 선택 영역을 감싸는 소괄호 삭제
