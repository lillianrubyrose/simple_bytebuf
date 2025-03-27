#ifndef SIMPLE_BYTEBUF_H
#define SIMPLE_BYTEBUF_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ByteBuf ByteBuf;

struct ByteBuf *bytebuf_create(uintptr_t capacity);

/**
 * # Safety
 *
 * We trust the caller to provide valid pointer and length
 */
struct ByteBuf *bytebuf_from_slice(const uint8_t *data, uintptr_t len);

/**
 * # Safety
 *
 * We trust the caller to provide a pointer created with bytebuf_create or bytebuf_from_slice
 */
void bytebuf_destroy(struct ByteBuf *buf);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_u8(struct ByteBuf *buf, uint8_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_u16(struct ByteBuf *buf, uint16_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_u32(struct ByteBuf *buf, uint32_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_u64(struct ByteBuf *buf, uint64_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_i8(struct ByteBuf *buf, int8_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_i16(struct ByteBuf *buf, int16_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_i32(struct ByteBuf *buf, int32_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_i64(struct ByteBuf *buf, int64_t *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_f32(struct ByteBuf *buf, float *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uint8_t bytebuf_read_f64(struct ByteBuf *buf, double *out);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_u8(struct ByteBuf *buf, uint8_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_u16(struct ByteBuf *buf, uint16_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_u32(struct ByteBuf *buf, uint32_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_u64(struct ByteBuf *buf, uint64_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_i8(struct ByteBuf *buf, int8_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_i16(struct ByteBuf *buf, int16_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_i32(struct ByteBuf *buf, int32_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_i64(struct ByteBuf *buf, int64_t val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_f32(struct ByteBuf *buf, float val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
void bytebuf_write_f64(struct ByteBuf *buf, double val);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer and data pointer + length
 */
void bytebuf_write_slice(struct ByteBuf *buf, const uint8_t *data, uintptr_t len);

/**
 * # Safety
 *
 * Proper bounds checking
 */
bool bytebuf_read_exact(struct ByteBuf *buf, uint8_t *out, uintptr_t len);

/**
 * # Safety
 *
 * We trust the caller to provide a valid buffer pointer
 */
uintptr_t bytebuf_len(const struct ByteBuf *buf);

#endif  /* SIMPLE_BYTEBUF_H */
