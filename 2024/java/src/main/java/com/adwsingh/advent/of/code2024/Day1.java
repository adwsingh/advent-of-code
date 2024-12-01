package com.adwsingh.advent.of.code2024;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day1 {

    public static void main(String[] args) throws IOException {
        List<Integer> a = new ArrayList<>();
        List<Integer> b = new ArrayList<>();
        Map<Integer, Integer> freq = new HashMap<>();
        try (var reader = Files.newBufferedReader(Paths.get(args[0]))) {
            reader.lines()
                    .map(str -> str.split("\\s+"))
                    .forEach(arr -> {
                        a.add(Integer.parseInt(arr[0]));
                        int x = Integer.parseInt(arr[1]);
                        b.add(x);
                        freq.merge(x, 1, Integer::sum);
                    });
        }
        Collections.sort(a);
        Collections.sort(b);
        int totalDistance = 0;
        int totalSimilarity = 0;
        for (int i = 0; i < a.size(); i++) {
            Integer x = a.get(i);
            totalDistance += Math.abs(x - b.get(i));
            totalSimilarity += x * freq.getOrDefault(x, 0);
        }
        System.out.println(totalDistance);
        System.out.println(totalSimilarity);
    }
}
