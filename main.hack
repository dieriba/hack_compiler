class Main {
    function void main() {
        String s;
        int energy, i;
        let s = Keyboard.readLine("Whats on your mind?");
        let energy = Keyboard.readInt("Whats your energy level?");
        let i = 0;
        let s = s.appendChar(33); // Appends the character '!'
        while (i < energy) {
        do Output.printString(s);
        do Output.println();
        let i = i + 1;
    }
    return;
    }
}