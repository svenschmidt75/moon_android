package com.svenschmidt.kitana.activities

import android.content.Intent
import android.os.Bundle
import android.view.MenuItem
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.GravityCompat
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.ViewModelProvider
import androidx.recyclerview.widget.LinearLayoutManager
import com.google.android.material.navigation.NavigationView
import com.svenschmidt.kitana.R
import com.svenschmidt.kitana.databinding.ActivityMainBinding
import com.svenschmidt.kitana.viewmodel.MainActivityViewModel
import kotlinx.android.synthetic.main.activity_main.*
import kotlinx.android.synthetic.main.activity_main_content.*
import kotlinx.android.synthetic.main.app_bar_main.*

class MainActivity : AppCompatActivity(), NavigationView.OnNavigationItemSelectedListener {

    lateinit var viewModel: MainActivityViewModel
    lateinit var binding: ActivityMainBinding

    companion object {
        init {
            // SS: load native libraries
            System.loadLibrary("moon_android")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = DataBindingUtil.setContentView(this, R.layout.activity_main)
        binding.lifecycleOwner = this

        viewModel = ViewModelProvider(this).get(MainActivityViewModel::class.java)
        binding.viewmodel = viewModel

        setupActionBar()

        drawer_main_view.setNavigationItemSelectedListener(this)

        initializeMainItems()
    }

    private fun initializeMainItems() {
        val items = viewModel.getMainItems()

        rv_items_list.layoutManager = LinearLayoutManager(this)
        rv_items_list.setHasFixedSize(true)

        val adapter = MainItemsAdapter(this@MainActivity, items)
        rv_items_list.adapter = adapter

//        adapter.setOnClickListener(object :
//            BoardItemsAdapter.OnClickListener {
//            override fun onClick(position: Int, model: Board) {
//                val intent = Intent(this@MainActivity, TaskListActivity::class.java)
//                intent.putExtra(Constants.DOCUMENT_ID, model.documentId)
//                startActivity(intent)
//            }
//        })
    }

    private fun setupActionBar() {

        setSupportActionBar(toolbar_main_activity)
        toolbar_main_activity.setNavigationIcon(R.drawable.ic_action_navigation_menu)

        toolbar_main_activity.setNavigationOnClickListener {
            toggleDrawer()
        }
    }

    override fun onBackPressed() {
        if (main_drawer_layout.isDrawerOpen(GravityCompat.START)) {
            main_drawer_layout.closeDrawer(GravityCompat.START)
        }
    }

    private fun toggleDrawer() {

        if (main_drawer_layout.isDrawerOpen(GravityCompat.START)) {
            main_drawer_layout.closeDrawer(GravityCompat.START)
        } else {
            main_drawer_layout.openDrawer(GravityCompat.START)
        }
    }

    override fun onNavigationItemSelected(item: MenuItem): Boolean {
        when (item.itemId) {
            R.id.date_time -> {
                var intent = Intent(this, DateTimeActivity::class.java)
                startActivity(intent)
            }
        }

        main_drawer_layout.closeDrawer(GravityCompat.START)
        return true
    }

}