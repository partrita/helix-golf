# snake_case to camelCase

모든 필드명을 camelCase로 변경합니다.

## Before

```js
const user_profile = {
  first_name: "John",
  last_name: "Doe",
  birth_date: "1990-05-15",
  email_address: "john_doe@example.com",
  phone_number: "555-123-4567",
  mailing_address: {
    street_name: "Main Street",
    house_number: 123,
    apartment_unit: "4B",
    zip_code: "10001",
    city_name: "New York",
  },
};
```

## After

```js
const userProfile = {
  firstName: "John",
  lastName: "Doe",
  birthDate: "1990-05-15",
  emailAddress: "john_doe@example.com",
  phoneNumber: "555-123-4567",
  mailingAddress: {
    streetName: "Main Street",
    houseNumber: 123,
    apartmentUnit: "4B",
    zipCode: "10001",
    cityName: "New York",
  },
};
```

## Command

```
%s_<enter>5)<alt-,>d~
```

1. `%` 전체 파일 선택
2. `s_<enter>` 모든 밑줄(_) 선택
3. `5)` 메인 선택 영역을 앞으로 5번 회전
4. `<alt-,>` 유지하려는 단독 밑줄(주 선택 영역) 제거
5. `d` 선택 영역 삭제
6. `~` 대소문자 전환
