# Swap Columns

두 열의 순서를 바꿉니다.

<!-- difficulty: advanced -->

## Before

```text
apple 1
banana 2
```

## After

```text
1 apple
2 banana
```

## Command

```
%<alt-s>S<space><enter>2<alt-(>
```

1. `%` 전체 파일 선택
1. `<alt-s>` 줄바꿈 기준으로 여러 선택 영역으로 분할
1. `S` 정규식 기준으로 선택 영역 분할
1. `<space>` 공백 문자 입력
1. `<enter>` 정규식 확정하고 각 줄을 두 열로 분할
1. `2<alt-(>` 두 선택 영역씩 묶어 내용 순서 회전
