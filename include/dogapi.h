/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#ifndef __DOGAPI_H__
#define __DOGAPI_H__

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  size_t length;
  size_t* values_length;
  char** keys;
  char*** values;
} dogapi_breeds_list_t;

extern int dogapi_breeds_list(dogapi_breeds_list_t* dogapi_breeds_list);

extern char** dogapi_images_by_breed(const char* breed, size_t* length);

extern char** dogapi_images_by_sub_breed(const char* breed, const char* sub_breed, size_t* length);

extern char** dogapi_multiple_random_images(int8_t images_number, size_t* length);

extern char** dogapi_multiple_random_images_by_breed(const char* breed, int64_t images_number, size_t* length);

extern char** dogapi_multiple_random_images_by_sub_breed(const char* breed, const char* sub_breed, int64_t images_number, size_t* length);

extern char* dogapi_random_image();

extern char* dogapi_random_image_by_breed(const char* breed);

extern char* dogapi_random_image_by_sub_breed(const char* breed, const char* sub_breed);

extern char** dogapi_sub_breeds_list(const char* breed, size_t* length);

extern void dogapi_string_free(char* ptr);

extern void dogapi_array_free(char** ptr, size_t length);

extern void dogapi_breeds_list_free(dogapi_breeds_list_t* ptr);

#ifdef __cplusplus
}
#endif

#endif // __DOGAPI_H__