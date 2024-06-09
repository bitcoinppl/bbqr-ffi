//
//  ContentView.swift
//  BBQrDemo
//
//  Created by Praveen Perera on 6/9/24.
//

import Bbqr
import SwiftUI

func largeString() -> String {
    String(repeating: "bacon", count: 1000)
}

func split() throws -> String {
    let large = Data(largeString().utf8)
    let options = Spl
    let split = try Split.tryFromData(bytes: large, fileType: FileType.unicodeText, options: options)
}

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
