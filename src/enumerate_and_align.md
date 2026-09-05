# Enumerate and Align

각 객체에 1부터 시작하여 증가하는 `rank` 필드를 추가하고, 보기 좋게 필드를 정렬합니다.

## Before

```js
[
  { word: "a", count: 2565 },
  { word: "and", count: 1777 },
  { word: "of", count: 1331 },
  { word: "that", count: 1263 },
  { word: "to", count: 1030 },
  { word: "in", count: 1027 },
  { word: "it", count: 754 },
  { word: "as", count: 730 },
  { word: "was", count: 687 },
  { word: "you", count: 652 },
  { word: "for", count: 630 },
];
```

## After

```js
[
  { rank:  1, word: "a",    count: 2565 },
  { rank:  2, word: "and",  count: 1777 },
  { rank:  3, word: "of",   count: 1331 },
  { rank:  4, word: "that", count: 1263 },
  { rank:  5, word: "to",   count: 1030 },
  { rank:  6, word: "in",   count: 1027 },
  { rank:  7, word: "it",   count:  754 },
  { rank:  8, word: "as",   count:  730 },
  { rank:  9, word: "was",  count:  687 },
  { rank: 10, word: "you",  count:  652 },
  { rank: 11, word: "for",  count:  630 },
];
```

## Command

```
%s\{<enter>a rank: <ctrl-r>

#,<esc>%s |\d+<enter>&
```

1. `%` 전체 파일 선택
1. `s` 선택 모드로 진입하여 패턴에 따라 하위 선택 영역 생성
1. `\{` 입력 후 `<enter>`를 눌러 모든 "\{"에 커서를 두고 단일 폭 선택 영역 생성
1. `a ` "\{" 뒤에서 삽입 모드로 진입
1. `rank: ` 입력
1. `<ctrl-r>` 누른 뒤 `#` 입력하여 1부터 증가하는 번호 삽입
1. `,` 입력
1. `<esc>` 일반 모드로 복귀
1. `%s` 다시 선택 모드로 진입
1. ` |\d+` 모든 공백과 숫자를 선택하는 정규식을 입력하고 `<enter>` 누르기
1. `&` 모든 선택 영역을 열에 맞춰 정렬 (숫자가 우측 정렬됨)
