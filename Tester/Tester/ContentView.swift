//
//  ContentView.swift
//  Tester
//
//  Created by Simone Margaritelli on 11/07/24.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "brain")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Button("RUN") {
                runMetalTests()
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
