# Indent Lines

여러 줄을 한 번에 들여씁니다.

<!-- difficulty: beginner -->

## Before

```text
apple
banana
cherry
```

## After

```text
  apple
  banana
  cherry
```

## Command

```
%<alt-s>I  <esc>
```

1. `%` 전체 파일 선택
1. `<alt-s>` 줄바꿈 기준으로 여러 선택 영역으로 분할
1. `I  ` 각 줄 앞에서 삽입 모드로 진입하고 두 칸 공백 입력
1. `<esc>` 일반 모드로 복귀
