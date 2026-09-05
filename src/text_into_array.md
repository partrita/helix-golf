# Text into Array

줄바꿈으로 구분된 데이터를 문자열 배열로 결합합니다.

## Before

```
Hello
This
Is
Helix
```

## After

```js
["Hello", "This", "Is", "Helix"]
```

## Command

```
%<alt-s>ms"<alt-J>i,<esc>xms[
```

1. `%` 전체 파일 선택
1. `<alt-s>` 줄바꿈을 기준으로 여러 선택 영역으로 분할
1. `ms"` 각 단어를 큰따옴표로 감싸기
1. `<alt-J>i,` 선택 영역 내부 라인들을 합치고 삽입된 공백을 선택한 뒤 쉼표(,) 삽입
1. `<esc>xms[` "[]"로 감싸기
