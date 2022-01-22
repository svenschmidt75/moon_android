package com.svenschmidt.kitana.core

import java.time.LocalDateTime

interface DateTimeProvider {
    fun start()
    fun stop()
    fun subscribe(observer: (dateTime: LocalDateTime) -> Unit): () -> Unit
    fun getCurrentLocalDateTime(): LocalDateTime
    fun setCurrentLocalDateTime(dateTime: LocalDateTime)
    fun getSystemLocalDateTime(): LocalDateTime
}