# CSV to Lines

쉼표로 구분된 한 줄을 여러 줄로 나눕니다.

<!-- difficulty: intermediate -->

## Before

```text
apple,banana,cherry
```

## After

```text
apple
banana
cherry
```

## Command

```
xs,<enter>c<ret><esc>
```

1. `x` 현재 줄 선택
1. `s` 정규식으로 일치 항목 선택
1. `,` 쉼표 문자 입력
1. `<enter>` 정규식 확정하고 모든 쉼표 선택
1. `c` 선택 영역을 변경
1. `<ret>` 줄바꿈 입력해 쉼표를 줄바꿈으로 교체
1. `<esc>` 일반 모드로 복귀
