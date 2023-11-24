/**
 * Resolved GIF metadata
 */
export class Probe {
  /**
   * Get the user-specified src of the file resolved
   *
   * Wrapper over a native property. Enumerable, not displayed on default
   * inspect
   *
   * @returns The src file path
   * @throws It just throws. You better catch the errors
   */
  public get src(): string;

  /**
   * Get the width of the file resolved
   *
   * Wrapper over a native property. Enumerable, not displayed on default
   * inspect
   *
   * @returns The width (px)
   * @throws It just throws. You better catch the errors
   */
  public get width(): number;

  /**
   * Get the height of the file resolved
   *
   * Wrapper over a native property. Enumerable, not displayed on default
   * inspect
   *
   * @returns The height (px)
   * @throws It just throws. You better catch the errors
   */
  public get height(): number;

  /**
   * Get the FPS (frames per second) of the file resolved
   *
   * Wrapper over a native property. Enumerable, not displayed on default
   * inspect
   *
   * @returns The framarate (FPS)
   * @throws It just throws. You better catch the errors
   */
  public get fps(): number;
}

/**
 * Abstraction for iterating over GIF frames
 */
export class Extract {
  /**
   * Iterable over the GIF frames. Each call does a read into preallocated
   * buffer
   *
   * @yields GIF frame in RGBA format
   * @throws It just throws. You better catch the errors
   */
  public [Symbol.iterator](): Iterator<Buffer>;
}

/**
 * Abstraction for generating a GIF
 */
export class Combine {
  /**
   * Allocates a buffer of required size, so you don't have to deal with it.
   *
   * Under the hood it basically allocates a `Buffer` of size
   * `probe.width * probe.height * 4`
   *
   * - This method is highly recommended to use so you don't get stuck once
   * you allocate a `Buffer` with invalid size
   * - It's also recommended **NOT TO CALL THIS METHOD TWICE**
   * - It's also recommended to only call this method **ONCE**
   *
   * @returns A `Buffer` with the appropriate size
   * @throws It just throws. You better catch the errors
   */
  public alloc(): Buffer;

  /**
   * Write a _generated_ frame to the GIF
   *
   * @param buf Pixel buffer. The format is RGBA
   *
   * @throws It just throws. You better catch the errors
   */
  public write(buf: Buffer): void;

  /**
   * Closes the write stream and reads the end result as a Buffer
   *
   * @returns Compiled GIF image
   * @throws It just throws. You better catch the errors
   */
  public finish(): Promise<Buffer>;
}

/**
 * Resolve GIF metadata
 *
 * @param src Path to the GIF file. **Relative to the `CWD`**
 *
 * @returns Resulting metadata
 * @throws It just throws. You better catch the errors
 */
export function probe(src: string): Promise<Probe>;

/**
 * Create GIF frame extractor instance
 *
 * @param probe Previously resolved GIF metadata
 *
 * @returns Instance for resolving frames
 * @throws It just throws. You better catch the errors
 */
export function extract(probe: Probe): Promise<Extract>;

/**
 * An optional parameter while combining a GIF
 */
export enum Repeat {
  Infinite = 0,
  Once = 1,
}

/**
 * Parameter descriptor for the `combine` function
 *
 * @throws Yes. Even an interface throws. Sure I'm just kidding
 */
export interface CombineOptions {
  /**
   * The width of the input buffer
   */
  width: number;
  /**
   * The height of the input buffer
   */
  height: number;
  /**
   * The frame rate of the output
   */
  fps: number;
  /**
   * The scale for the output
   */
  scale?: [number, number] | null;
  repeat?: Repeat | number | null;
}

/**
 * Combine RGBA Buffers frames into a single GIF
 *
 * @param probe Previously resolved GIF metadata
 * @param options Parameters to rely on
 *
 * @returns Instance for generating a GIF
 * @throws It just throws. You better catch the errors
 */
export function combine(options: CombineOptions): Promise<Combine>;
