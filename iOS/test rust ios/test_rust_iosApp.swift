//
//  test_rust_iosApp.swift
//  test rust ios
//
//  Created by pichane on 03/12/2023.
//

import SwiftUI
import os.log

@main
struct test_rust_iosApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
    
    init() {
        let log = OSLog(subsystem: "com.yourcompany.yallah", category: "app-lifecycle")
        os_log("Yallah App initialized", log: log)
        
        
        let result = rust_hello("yattah")
        let swift_result = String(cString: result!)
        rust_hello_free(UnsafeMutablePointer(mutating: result))
        print(swift_result)
    }
}
