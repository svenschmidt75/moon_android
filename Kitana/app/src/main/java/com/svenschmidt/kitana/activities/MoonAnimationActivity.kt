package com.svenschmidt.kitana.activities

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.ViewModelProvider
import com.svenschmidt.kitana.R
import com.svenschmidt.kitana.databinding.ActivityMoonAnimationBinding
import com.svenschmidt.kitana.databinding.ActivityMoonBinding
import com.svenschmidt.kitana.viewmodel.MoonActivityViewModel
import com.svenschmidt.kitana.viewmodel.MoonAnimationActivityViewModel
import kotlinx.android.synthetic.main.activity_moon.*
import kotlinx.android.synthetic.main.activity_moon_animation.*

class MoonAnimationActivity : AppCompatActivity() {
    lateinit var viewModel: MoonAnimationActivityViewModel
    lateinit var binding: ActivityMoonAnimationBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = DataBindingUtil.setContentView(this, R.layout.activity_moon_animation)
        binding.lifecycleOwner = this

        viewModel = ViewModelProvider(this).get(MoonAnimationActivityViewModel::class.java)
        binding.viewmodel = viewModel

        setupActionBar()
    }

    private fun setupActionBar() {

        setSupportActionBar(moon_animation_activity_toolbar)
        supportActionBar!!.setDisplayHomeAsUpEnabled(true)
        moon_animation_activity_toolbar.setNavigationOnClickListener {
            onBackPressed()
        }
    }
}