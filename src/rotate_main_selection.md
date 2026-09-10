# Rotate Main Selection

주 선택 영역을 옮겨 마지막 숫자만 바꿉니다.

<!-- difficulty: intermediate -->

## Before

```text
a1 a2 a3
```

## After

```text
a1 a2 a9
```

## Command

```
%s\d<enter>)),c9<esc>
```

1. `%` 전체 파일 선택
1. `s` 정규식으로 일치 항목 선택
1. `\d` 숫자 패턴 입력
1. `<enter>` 정규식 확정하고 모든 숫자 선택
1. `)` 주 선택 영역을 앞으로 회전
1. `)` 한 번 더 회전해 마지막 숫자로 이동
1. `,` 주 선택 영역만 남기기
1. `c` 선택 영역을 변경
1. `9` 새 숫자 입력
1. `<esc>` 일반 모드로 복귀
