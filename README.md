# libdogapi

[![](https://img.shields.io/github/v/tag/thechampagne/libdogapi?label=version)](https://github.com/thechampagne/libdogapi/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libdogapi)](https://github.com/thechampagne/libdogapi/blob/main/LICENSE)

Dog API client for **C**.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libdogapi.git
```
#### 2. Navigate to the root
```
cd libdogapi
```
#### 3. Build the project
```
cargo build
```

### API

```c
typedef struct {
  size_t length;
  size_t* values_length;
  char** keys;
  char*** values;
} dogapi_breeds_list_t;

int dogapi_breeds_list(dogapi_breeds_list_t* dogapi_breeds_list);

char** dogapi_images_by_breed(const char* breed, size_t* length);

char** dogapi_images_by_sub_breed(const char* breed, const char* sub_breed, size_t* length);

char** dogapi_multiple_random_images(int8_t images_number, size_t* length);

char** dogapi_multiple_random_images_by_breed(const char* breed, int64_t images_number, size_t* length);

char** dogapi_multiple_random_images_by_sub_breed(const char* breed, const char* sub_breed, int64_t images_number, size_t* length);

char* dogapi_random_image();

char* dogapi_random_image_by_breed(const char* breed);

char* dogapi_random_image_by_sub_breed(const char* breed, const char* sub_breed);

char** dogapi_sub_breeds_list(const char* breed, size_t* length);

void dogapi_string_free(char* ptr);

void dogapi_array_free(char** ptr, size_t length);

void dogapi_breeds_list_free(dogapi_breeds_list_t* ptr);
```

### References
 - [dogapi](https://github.com/thechampagne/dogapi-rust)

### License

This repo is released under the [Apache License 2.0](https://github.com/thechampagne/libdogapi/blob/main/LICENSE).

```
 Copyright 2022 XXIV
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
```