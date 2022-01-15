package com.svenschmidt.kitana

import android.app.Application
import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import org.junit.Test
import org.junit.Assert.*
import org.mockito.Mockito

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
class DateTimeViewModelTest {

    @Test
    fun addition_isCorrect() {
        // Arrange
        val applicationMock = Mockito.mock(Application::class.java)
        val model = DateTimeViewModel(applicationMock)

        assertEquals(4, 2 + 2)
    }
}