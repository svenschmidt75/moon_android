package com.svenschmidt.kitana.core

import java.time.LocalDateTime
import java.util.*

interface DateTimeProvider {
    fun start(observer: Observer)
    fun stop()
    fun getLocalDateTime(): LocalDateTime
    fun setLocalDateTime(dateTime: LocalDateTime)
}

class DateTimeProviderImpl: DateTimeProvider, Observable() {

    companion object {
        const val SECOND: Long = 1000
    }

    private var isRunning = false
    private var timer = Timer()
    lateinit var dateTime: LocalDateTime

    override fun stop() {
        timer.cancel()
        isRunning = false
    }

    override fun getLocalDateTime(): LocalDateTime {
        return LocalDateTime.now()
    }

    override fun setLocalDateTime(dateTime: LocalDateTime) {
        this@DateTimeProviderImpl.dateTime = dateTime
    }

    override fun start(observer: Observer) {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                dateTime = dateTime.plusSeconds(1)
                observer.update(this@DateTimeProviderImpl, dateTime)
            }

        }, 0, SECOND)
    }

}