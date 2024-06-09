//
//  ContentView.swift
//  BBQrDemo
//
//  Created by Praveen Perera on 6/9/24.
//

import Bbqr
import SwiftUI

func largeString() -> String {
    String(repeating: "bacon", count: 25)
}

func split() throws -> [String] {
    let large = Data(largeString().utf8)

    // EXAMPLE DEFAULT OPTIONS
    // let options = defaultSplitOptions()
    // let options = SplitOptions(encoding: Encoding.zlib, minVersion: Version.v01, maxVersion: Version.v40)

    let options = SplitOptions(encoding: Encoding.hex, minVersion: Version.v01, maxVersion: Version.v02)
    let split = try Split.tryFromData(bytes: large, fileType: FileType.unicodeText, options: options)

    return split.parts()
}

func continousJoiner(parts: [String]) throws -> String {
    let continousJoiner = ContinuousJoiner()

    for part in parts {
        switch try continousJoiner.addPart(part: part) {
        case .notStarted:
            print("not started")
        case .inProgress(let partsLeft):
            print("added item")
        case .complete(let joined):
            return String(decoding: joined.data(), as: UTF8.self)
        }
    }

    return ""
}

struct ContentView: View {
    var body: some View {
        VStack {
            Text("split")
                .padding(.bottom, 10)
            try! ForEach(split(), id: \.self) {
                Text($0).padding(2)
            }
            Divider().padding(30)
            Text("joined")
                .padding(.bottom, 10)
            try! Text(continousJoiner(parts: split()))
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
