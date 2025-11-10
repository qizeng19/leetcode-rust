const RANGE = 3000;
class RecentCounter {
    private queue: number[];
    constructor() {
        this.queue = []
    }

    ping(t: number): number {
        while (this.queue.length > 0 && this.queue[0] < t - RANGE) {
            this.queue.shift();
        }
        this.queue.push(t);
        return this.queue.length;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * var obj = new RecentCounter()
 * var param_1 = obj.ping(t)
 */