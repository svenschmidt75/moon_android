package com.svenschmidt.kitana

import android.app.DatePickerDialog
import android.app.TimePickerDialog
import android.os.Bundle
import android.view.View
import android.view.View.VISIBLE
import android.widget.Button
import android.widget.DatePicker
import android.widget.TimePicker
import androidx.appcompat.app.AppCompatActivity
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.ViewModelProvider
import com.google.android.material.button.MaterialButton
import com.google.android.material.checkbox.MaterialCheckBox
import com.svenschmidt.kitana.databinding.ActivityDateTimeBinding
import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import java.util.*

class DateTimeActivity : AppCompatActivity() {

    lateinit var viewModel: DateTimeViewModel
    lateinit var binding: ActivityDateTimeBinding
    private var cal = Calendar.getInstance()


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = DataBindingUtil.setContentView(this, R.layout.activity_date_time)
        binding.lifecycleOwner = this

        viewModel = ViewModelProvider(this).get(DateTimeViewModel::class.java)
        binding.viewmodel = viewModel

        binding.localDateAndTimeSelectDate.setOnClickListener {
            DatePickerDialog(
                this,
                object: DatePickerDialog.OnDateSetListener {
                    override fun onDateSet(
                        view: DatePicker?,
                        year: Int,
                        month: Int,
                        dayOfMonth: Int
                    ) {
                        viewModel.setDate(year, month, dayOfMonth)
                    }
                },
                cal.get(Calendar.YEAR),
                cal.get(Calendar.MONTH),
                cal.get(Calendar.DAY_OF_MONTH)
            ).show()
        }

        binding.localDateAndTimeSelectTime.setOnClickListener {
            TimePickerDialog(
                this,
                object: TimePickerDialog.OnTimeSetListener {
                    override fun onTimeSet(view: TimePicker?, hourOfDay: Int, minute: Int) {
                        viewModel.setTime(hourOfDay, minute)
                    }
                },
                cal.get(Calendar.HOUR),
                cal.get(Calendar.MINUTE),
                false
            ).show()
        }


    }

}