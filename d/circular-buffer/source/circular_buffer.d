
module circular;

class Buffer(T) {
  int start = 0;
  int end = 0;
  int size;
  int count;
  T[] back;
  this(int s) {
    back = new T[s];
    size = s;
  }

  void shiftEnd(int i) {
    end += i;
    if (end <= 0) {
      end = size - end;
    } else if (end >= size) {
      end = end - size;
    }
  }

  void shiftStart(int i) {
    start += i;
    if (start <= 0) {
      start = size - start;
    } else if (start >= size) {
      start = start - size;
    }
  }

  T pop() {
    if (count <= 0) {
        throw new Exception("Empty buffer");
    } else {
      shiftStart(1);
      count--;
      return back[start];
    }
  }
  void push(T item) {
    if (count < size) {
      shiftEnd(1);
      count++;
      back[end] = item;
    } else {
      throw new Exception("Full buffer");
    }
  }
  void forcePush(T item) {
    shiftEnd(1);
    back[end] = item;
    if (count < size) {
      count++;
    } else {
      shiftStart(1);
    }
  }
  void clear() {
    back = new T[size];
    count = 0;
    end = 0;
  }
}

unittest
{
import std.exception : assertThrown;

immutable int allTestsEnabled = 1;

// test read empty buffer
{
	auto myBuffer = new Buffer!(int)(1UL);
	assertThrown(myBuffer.pop(), "Empty buffer should throw exception if popped!");
}

static if (allTestsEnabled)
{

// test write and read back one item
{
	auto myBuffer = new Buffer!(char)(1);
	myBuffer.push('1');
	assert(myBuffer.pop() == '1');
}

// test write and read back multiple items
{
	auto myBuffer =  new Buffer!(char)(2);
	myBuffer.push('1');
	myBuffer.push('2');
	assert(myBuffer.pop() == '1');
	assert(myBuffer.pop() == '2');
}

// test clearing the buffer
{
	auto myBuffer = new Buffer!(char)(3);
	myBuffer.push('1');
	myBuffer.push('2');
	myBuffer.push('3');

	myBuffer.clear();
	assertThrown(myBuffer.pop(), "Empty buffer should throw exception if popped!");
}

// test alternate write and read
{
	auto myBuffer = new Buffer!(char)(2);
	myBuffer.push('1');
	assert(myBuffer.pop() == '1');
	myBuffer.push('2');
	assert(myBuffer.pop() == '2');
}

// test read back oldest item
{
	auto myBuffer = new Buffer!(char)(4);
	myBuffer.push('1');
	myBuffer.push('2');
	myBuffer.pop();
	myBuffer.push('3');
	myBuffer.pop();

	assert(myBuffer.pop() == '3');
}

// test write buffer
{
	auto myBuffer = new Buffer!(char)(3);
	myBuffer.push('1');
	myBuffer.push('2');
	myBuffer.push('3');

	assertThrown(myBuffer.push('4'), "Full buffer should throw exception if new element pushed!");
}

// test forcePush full buffer
{
	auto myBuffer = new Buffer!(char)(3);
	myBuffer.push('1');
	myBuffer.push('2');
	myBuffer.push('3');

	myBuffer.forcePush('A');
	assert(myBuffer.pop() == '2');
	assert(myBuffer.pop() == '3');
	assert(myBuffer.pop() == 'A');
}

// test forcePush non-full buffer
{
	auto myBuffer = new Buffer!(int)(2);
	myBuffer.forcePush(1000);
	myBuffer.forcePush(2000);

	assert(myBuffer.pop() == 1000);
	assert(myBuffer.pop() == 2000);
}

// test alternate read and forcePush
{
	auto myBuffer = new Buffer!(char)(5);
	myBuffer.push('1');
	myBuffer.push('2');
	myBuffer.push('3');
	myBuffer.pop();
	myBuffer.pop();

	myBuffer.push('4');
	myBuffer.pop();

	myBuffer.push('5');
	myBuffer.push('6');
	myBuffer.push('7');
	myBuffer.push('8');
	myBuffer.forcePush('A');
	myBuffer.forcePush('B');

	assert(myBuffer.pop() == '6');
	assert(myBuffer.pop() == '7');
	assert(myBuffer.pop() == '8');
	assert(myBuffer.pop() == 'A');
	assert(myBuffer.pop() == 'B');
	assertThrown(myBuffer.pop(), "Empty buffer should throw exception if popped!");
}

}

}
