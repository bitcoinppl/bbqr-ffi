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

// continousJoiner allows you to join parts one by one as the come in from the QR code scanner
func continousJoiner(parts: [String]) throws -> String {
    let continousJoiner = ContinuousJoiner()

    for part in parts {
        switch try continousJoiner.addPart(part: part) {
        case .notStarted:
            print("not started")
        case .inProgress(let partsLeft):
            print("added item, \(partsLeft) parts left")
        case .complete(let joined):
            return String(decoding: joined.data(), as: UTF8.self)
        }
    }

    return ""
}

// if you already have all the parts you can join them together at once using Joined
func bulkJoin(parts: [String]) throws -> String {
    let joined = try Joined.tryFromParts(parts: parts)
    return String(decoding: joined.data(), as: UTF8.self)
}

struct ContentView: View {
    var body: some View {
        VStack {
            ScrollView {
                Text("split")
                    .padding(.bottom, 10)
                try! ForEach(split(), id: \.self) {
                    Text($0).padding(2)
                }

                Divider().padding(30)

                Text("joined using continous joiner")
                    .padding(.bottom, 10)
                try! Text(continousJoiner(parts: split()))

                Divider().padding(30)
                Text("joined at once")
                    .padding(.bottom, 10)
                try! Text(bulkJoin(parts: split()))
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
