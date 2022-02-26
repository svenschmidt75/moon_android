package com.svenschmidt.kitana.core

import java.time.ZonedDateTime
import java.util.*

class DateTimeProviderImpl : DateTimeProvider {

    companion object {
        const val SECOND: Long = 1000
    }

    private var subscribers = mutableListOf<(dateTime: ZonedDateTime) -> Unit>()
    private var isRunning = false
    private var timer = Timer()
    var dateTime: ZonedDateTime

    init {
        dateTime = ZonedDateTime.now()
    }

    override fun stop() {
        timer.cancel()
        isRunning = false
    }

    override fun getSystemLocalDateTime(): ZonedDateTime {
        return ZonedDateTime.now()
    }

    override fun getCurrentLocalDateTime(): ZonedDateTime {
        return dateTime
    }

    override fun setCurrentLocalDateTime(dateTime: ZonedDateTime) {
        this@DateTimeProviderImpl.dateTime = dateTime
    }

    override fun subscribe(observer: (dateTime: ZonedDateTime) -> Unit): () -> Unit {
        subscribers.add(observer)
        return {
            subscribers.remove(observer)
        };
    }

    override fun start() {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                dateTime = dateTime.plusSeconds(1)

                for (subscriber in subscribers) {
                    subscriber.invoke(dateTime)
                }
            }

        }, 0, SECOND)
    }

}