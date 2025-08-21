package org.example;

import java.util.Scanner;

public class Day03 {
    void init() {
        System.out.println("What is your name?");
        Scanner scanner = new Scanner(System.in);
        String name = scanner.nextLine();
        System.out.println("Can you input an int?");
        int intNumber = scanner.nextInt();
        System.out.printf("Your int is %s%n", intNumber);

        System.out.printf("Your name is %s%n", name);
    }
}
