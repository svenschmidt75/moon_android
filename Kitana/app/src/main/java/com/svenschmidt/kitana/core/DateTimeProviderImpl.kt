package com.svenschmidt.kitana.core

import java.util.*

interface DateTimeProvider {
    fun start(observer: Observer)
    fun stop()
    fun getLocalUTCTimeMillis(): Long
}

class DateTimeProviderImpl: DateTimeProvider, Observable() {

    companion object {
        const val SECOND: Long = 1000
    }

    private var isRunning = false
    private var timer = Timer()

    override fun stop() {
        timer.cancel()
        isRunning = false
    }

    override fun getLocalUTCTimeMillis(): Long {
        return System.currentTimeMillis()
    }

    override fun start(observer: Observer) {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                val nowUTC = getLocalUTCTimeMillis()
                observer.update(this@DateTimeProviderImpl, nowUTC)
            }

        }, 0, SECOND)
    }

}