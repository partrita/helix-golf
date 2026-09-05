# Reverse Golf Example

Helix Golf 예제의 "Before"와 "After" 케이스를 서로 맞바꿉니다.

## Before

````md
# snake_case to camelCase

Rename all fields to be camelCase.

## Before

```js
const user_profile = {first_name: "John"};
```

## After

```js
const userProfile = {firstName: "John"};
```
````

## After

````md
# camelCase to snake_case

Rename all fields to be snake_case.

## Before

```js
const userProfile = {firstName: "John"};
```

## After

```js
const user_profile = {first_name: "John"};
```
````

## Command

```
ebyxb*Rv""Nn<alt-)>

%s`+j<enter>f;<alt-(>
```

1. `eb` 다음 단어를 선택하고 공백 제거
1. `y` 선택한 단어를 복사하여 " (큰따옴표) 레지스터에 저장
1. `xb` 줄의 마지막 단어 선택
1. `*` 현재 선택 영역을 검색 패턴으로 설정
1. `R` 선택한 단어를 이전에 복사한 선택 항목으로 교체
1. `v` 선택 모드 진입
1. `""` " (큰따옴표) 레지스터를 지정. `N`을 누르면 레지스터에 저장된 단어의 이전 일치 항목에 새 선택 영역 추가
1. `n` 이전에 지정된 검색 패턴의 다음 일치 항목에 새 선택 영역 추가
1. `<alt-)>` 선택 영역 내용의 순서를 앞으로 회전
1. `%` 파일 전체 내용 선택
1. `s` 정규식 일치 기준으로 하위 선택 영역 생성. `` `+j ``를 정규식으로 입력하고 `<enter>`를 눌러 모든 일치 항목 선택
1. 여전히 선택 모드이므로 `f;`를 입력하여 각 커서가 다음 세미콜론(;)까지(포함) 선택하도록 이동
1. `<alt-(>` 선택 영역 내용의 순서를 뒤로 회전 
