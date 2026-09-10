# Function into Class

<!-- difficulty: advanced -->

3개의 함수를 3개의 메서드를 가진 클래스로 변환합니다.

## Before

```py
def calculate_area(length, width):
    result = length * width
    return result

def calculate_perimiter(length, width):
    result = 2 * (length + width)
    return result

def calculate_volume(length, width, height):
    result = length * width * height
    return result
```

## After

```py
class Calculator:
    @staticmethod
    def get_area(len, wid):
        return len * wid

    @staticmethod
    def get_perimiter(len, wid):
        return 2 * (len + wid)

    @staticmethod
    def get_volume(len, wid, hei):
        return len * wid * hei
```

## Command

```
%scalculate<enter>cget<esc>

O@staticmethod<esc>jj

vglyx<alt-d>xbRkxx

slength|width|height<enter>

bllled%>O<backspace>

class Calculator:
```

1. `%` 전체 파일 선택
1. `s` 현재 선택 영역 내에서 검색하여 하위 선택 영역 생성. `calculate` 입력 후 `<enter>`를 눌러 해당 단어의 모든 항목 선택
1. `c` 누른 후 `get` 입력하여 각 "calculate" 단어를 "get"으로 변경
1. `<esc>` 일반 모드로 복귀
1. `O` 각 커서 위에 빈 줄 생성 후 다음 입력:

    ```
    @staticmethod
    ```

1. `<esc>` 일반 모드로 복귀
1. `jj` 각 커서를 아래로 두 줄 이동
1. `vgl` 각 커서 이후 줄의 나머지 부분 선택
1. `y` 각 선택 영역 복사
1. `x<alt-d>` 각 커서의 줄을 선택하고 선택 영역을 복사하지 않고 줄 삭제
1. `xb` 각 커서 줄의 마지막 단어 선택
1. `R` 각 선택 영역을 이전에 복사한 선택 항목으로 교체
1. `kxx` 각 커서를 한 줄 위로 이동하고 해당 줄과 아래 줄 선택
1. `s` 정규식으로 하위 선택 영역 생성. `length|width|height` 입력 후 `<enter>`를 눌러 length, width, height 단어 선택
1. `b` 각 단어의 시작 부분으로 이동
1. 매개변수에서 처음 3글자만 유지하고 나머지를 버리기 위해 `lll`로 4번째 문자로 이동
1. `e` 각 단어 끝까지 선택 후 `d`로 삭제
1. `%` 전체 파일 선택 후 `>`로 들여쓰기
1. `O` 위에 새 줄 생성하고 삽입 모드 진입, `<backspace>`로 추가 탭 삭제
1. 다음 내용 입력:

    ```
    class Calculator:
    ```
