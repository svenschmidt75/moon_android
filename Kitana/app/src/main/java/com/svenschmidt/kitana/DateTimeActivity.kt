package com.svenschmidt.kitana

import android.app.DatePickerDialog
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.DatePicker
import androidx.appcompat.app.AppCompatActivity
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.ViewModelProvider
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

        binding.localDateAndTimeSelectDate.setOnClickListener { it ->
            println()
        }

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
//                        viewModel
                    }
                },
                cal.get(Calendar.YEAR),
                cal.get(Calendar.MONTH),
                cal.get(Calendar.DAY_OF_MONTH)
            ).show()
        }

    }

}