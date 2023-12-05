import SwiftUI
import os.log

struct ContentView: View {
    @State private var resultText: String = "Loading..." // Initial placeholder text
    @State private var input: String = "And then I" // Initial placeholder text
    @State private var aze: String = "" // Initial text is empty

    var body: some View {
        VStack {
            Text("Enter a Text") // Title
                .font(.title)
                .padding(.top, 20)

            HStack {
                TextField("Type here...", text: $input) // EditText
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding()

                Button("Click Me") {
                    // Log the text from EditText
                    print("Entered Text: \(input)")
                    performInitialization(resource: input)
                }
                .padding()
            }
            .padding()

            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
                .padding(.bottom, 10)

            Text(resultText) // Display the result here
        }
        .padding()
        .onAppear {
            performInitialization(resource: input)
        }
    }

    func performInitialization(resource: String) {  
        // Perform the initialization logic here
        let log = OSLog(subsystem: "com.yourcompany.yallah", category: "app-lifecycle")
        os_log("Yallah App initialized", log: log)
        
        if let path1 = Bundle.main.path(forResource: "hgwells", ofType: "txt") {
            // Use the 'path' variable, which contains the full path to the
            guard
                // let path1 = Bundle.main.path(forResource: "hgwells", ofType: "txt"),
                let path2 = Bundle.main.path(forResource: "dickens", ofType: "txt"),
                let path3 = Bundle.main.path(forResource: "franklin", ofType: "txt")
            else {
                // Handle the case where at least one file is not found
                print("At least one file not found.")
                return
            }

            print("resource : \(resource)")
            print("File path: \(path1)")
            print("File path: \(path2)")
            print("File path: \(path3)")
            
            let result = rust_helloo("\(resource)",  "\(path1)", "\(path2)", "\(path3)")
            let swiftResult = String(cString: result!)
            rust_hello_free(UnsafeMutablePointer(mutating: result))
            print(swiftResult)
            
            // Update the resultText with the Rust result
            resultText = "And then I " + swiftResult
        } else {
            print("File not found in the bundle.")
        }
               
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
