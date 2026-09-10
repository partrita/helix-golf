# CSV to SQL

<!-- difficulty: advanced -->

## Before

```csv
id 1,Item 1,cost 1,location 1
id 2,Item 2,cost 2,location 2
id 10,Item 10,cost 10,location 10
```

## After

```sql
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 1','Item 1','cost 1','Location 1');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 2','Item 2','cost 2','Location 2');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 10','Item 10','cost 10','Location 10');
```

## Command

```
%<alt-s>"yys\d<enter>

dhhbms

``x_ms(IINSERT INTO `

database<esc>

a.`table<esc>la <esc>

AVALUES (<esc>

"yPS,<enter>ms'A;<esc>Fl;~
```

1. `%` 전체 파일 선택
1. `<alt-s>` 줄바꿈을 기준으로 여러 선택 영역으로 분할
1. `"yy` 나중에 사용하기 위해 "y" 레지스터에 복사(yank)
1. `s` 누르고 패턴 `\d` 입력 후 `<enter>`를 눌러 모든 숫자에 선택 영역 생성
1. `d` 선택 영역 삭제 (모든 숫자 제거)
1. `hh` 각 단어의 끝에 위치하도록 뒤로 2글자 이동
1. `b` 각 단어의 시작 부분까지 선택하여 모든 단어를 선택
1. `` ms` `` 각 단어를 백틱으로 감싸기
1. `` ` `` 모든 문자를 소문자로 변환
1. `x` 각 줄을 선택한 다음 `_`로 끝 공백 제거
1. `ms(` 각 줄을 괄호로 감싸기
1. `I` 각 줄의 맨 앞에서 삽입 모드 진입
1. 다음 내용 입력:

    ```
    INSERT INTO `database
    ```

1. `<esc>` 일반 모드로 복귀
1. `a` 백틱 뒤에서 삽입 모드로 진입한 후 다음 입력:

    ```
    .`table
    ```

1. `<esc>` 일반 모드로 복귀 후 `la`로 여는 괄호 바로 앞에서 삽입 모드 진입
1. 공백 ` ` 추가 후 다시 `<esc>`로 일반 모드 복귀
1. `A` 각 줄 끝에서 삽입 모드로 진입하여 다음 입력:

    ```
    VALUES (
    ```

1. `<esc>` 눌러 삽입 모드 종료 (닫는 괄호 위치에 커서 배치)
1. `"yP` "y" 레지스터에 복사해 둔 항목들 붙여넣기
1. `S,<enter>` 각 쉼표를 기준으로 현재 선택 영역을 여러 개로 분할
1. `ms'` 각 항목을 작은따옴표로 감싸기
1. `A;` 각 줄 끝에 세미콜론 추가
1. `<esc>` 일반 모드로 복귀 후 `Fl`로 각 "location"의 소문자 "l"에 커서 위치
1. `;` 각 선택 영역을 단일 폭 선택 영역으로 축소
1. `~` 각 "l"을 "L"로 대소문자 전환
