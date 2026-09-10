# Replace Punctuation

여러 줄의 앞 기호를 한 번에 바꿉니다.

<!-- difficulty: beginner -->

## Before

```text
- apple
- banana
- cherry
```

## After

```text
* apple
* banana
* cherry
```

## Command

```
%s-<ret>r*
```

1. `%` 전체 파일 선택
1. `s` 정규식으로 일치 항목 선택
1. `-` 하이픈 문자 입력
1. `<ret>` 정규식 확정하고 모든 하이픈 선택
1. `r` 선택 영역을 한 글자로 교체
1. `*` 별표 입력
