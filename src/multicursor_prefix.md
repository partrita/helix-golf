# Multicursor Prefix

아래줄에 커서를 복제해 세 줄에 동시에 입력합니다.

<!-- difficulty: intermediate -->

## Before

```text
apple
banana
cherry
```

## After

```text
fruit: apple
fruit: banana
fruit: cherry
```

## Command

```
xCCIfruit: <esc>
```

1. `x` 현재 줄 선택
1. `C` 아래줄에 커서 복제
1. `C` 한 번 더 복제해 세 커서 만들기
1. `Ifruit: ` 각 줄 앞에서 삽입 모드로 진입하고 접두사 입력
1. `<esc>` 일반 모드로 복귀
