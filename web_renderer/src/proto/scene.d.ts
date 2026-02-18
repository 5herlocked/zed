import * as $protobuf from "protobufjs";
import Long = require("long");
/** Namespace zed. */
export namespace zed {

    /** Namespace scene. */
    namespace scene {

        /** Properties of a FrameMessage. */
        interface IFrameMessage {

            /** FrameMessage frameId */
            frameId?: (number|Long|null);

            /** FrameMessage viewportWidth */
            viewportWidth?: (number|null);

            /** FrameMessage viewportHeight */
            viewportHeight?: (number|null);

            /** FrameMessage scaleFactor */
            scaleFactor?: (number|null);

            /** FrameMessage atlasEntries */
            atlasEntries?: (zed.scene.IAtlasEntry[]|null);

            /** FrameMessage scene */
            scene?: (zed.scene.ISceneBody|null);

            /** FrameMessage backgroundColor */
            backgroundColor?: (zed.scene.IHsla|null);

            /** FrameMessage themeHints */
            themeHints?: (zed.scene.IThemeHints|null);
        }

        /** Represents a FrameMessage. */
        class FrameMessage implements IFrameMessage {

            /**
             * Constructs a new FrameMessage.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IFrameMessage);

            /** FrameMessage frameId. */
            public frameId: (number|Long);

            /** FrameMessage viewportWidth. */
            public viewportWidth: number;

            /** FrameMessage viewportHeight. */
            public viewportHeight: number;

            /** FrameMessage scaleFactor. */
            public scaleFactor: number;

            /** FrameMessage atlasEntries. */
            public atlasEntries: zed.scene.IAtlasEntry[];

            /** FrameMessage scene. */
            public scene?: (zed.scene.ISceneBody|null);

            /** FrameMessage backgroundColor. */
            public backgroundColor?: (zed.scene.IHsla|null);

            /** FrameMessage themeHints. */
            public themeHints?: (zed.scene.IThemeHints|null);

            /**
             * Creates a new FrameMessage instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FrameMessage instance
             */
            public static create(properties?: zed.scene.IFrameMessage): zed.scene.FrameMessage;

            /**
             * Encodes the specified FrameMessage message. Does not implicitly {@link zed.scene.FrameMessage.verify|verify} messages.
             * @param message FrameMessage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IFrameMessage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FrameMessage message, length delimited. Does not implicitly {@link zed.scene.FrameMessage.verify|verify} messages.
             * @param message FrameMessage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IFrameMessage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FrameMessage message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FrameMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.FrameMessage;

            /**
             * Decodes a FrameMessage message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FrameMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.FrameMessage;

            /**
             * Verifies a FrameMessage message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FrameMessage message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FrameMessage
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.FrameMessage;

            /**
             * Creates a plain object from a FrameMessage message. Also converts values to other types if specified.
             * @param message FrameMessage
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.FrameMessage, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FrameMessage to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for FrameMessage
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a ThemeHints. */
        interface IThemeHints {

            /** ThemeHints appearance */
            appearance?: (string|null);

            /** ThemeHints backgroundRgb */
            backgroundRgb?: (number|null);

            /** ThemeHints backgroundCss */
            backgroundCss?: (string|null);

            /** ThemeHints backgroundAppearance */
            backgroundAppearance?: (string|null);
        }

        /** Represents a ThemeHints. */
        class ThemeHints implements IThemeHints {

            /**
             * Constructs a new ThemeHints.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IThemeHints);

            /** ThemeHints appearance. */
            public appearance: string;

            /** ThemeHints backgroundRgb. */
            public backgroundRgb: number;

            /** ThemeHints backgroundCss. */
            public backgroundCss: string;

            /** ThemeHints backgroundAppearance. */
            public backgroundAppearance: string;

            /**
             * Creates a new ThemeHints instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ThemeHints instance
             */
            public static create(properties?: zed.scene.IThemeHints): zed.scene.ThemeHints;

            /**
             * Encodes the specified ThemeHints message. Does not implicitly {@link zed.scene.ThemeHints.verify|verify} messages.
             * @param message ThemeHints message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IThemeHints, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ThemeHints message, length delimited. Does not implicitly {@link zed.scene.ThemeHints.verify|verify} messages.
             * @param message ThemeHints message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IThemeHints, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ThemeHints message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ThemeHints
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.ThemeHints;

            /**
             * Decodes a ThemeHints message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ThemeHints
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.ThemeHints;

            /**
             * Verifies a ThemeHints message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ThemeHints message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ThemeHints
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.ThemeHints;

            /**
             * Creates a plain object from a ThemeHints message. Also converts values to other types if specified.
             * @param message ThemeHints
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.ThemeHints, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ThemeHints to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for ThemeHints
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a SceneBody. */
        interface ISceneBody {

            /** SceneBody shadows */
            shadows?: (zed.scene.IShadow[]|null);

            /** SceneBody quads */
            quads?: (zed.scene.IQuad[]|null);

            /** SceneBody underlines */
            underlines?: (zed.scene.IUnderline[]|null);

            /** SceneBody monochromeSprites */
            monochromeSprites?: (zed.scene.IMonochromeSprite[]|null);

            /** SceneBody subpixelSprites */
            subpixelSprites?: (zed.scene.ISubpixelSprite[]|null);

            /** SceneBody polychromeSprites */
            polychromeSprites?: (zed.scene.IPolychromeSprite[]|null);

            /** SceneBody paths */
            paths?: (zed.scene.IPath[]|null);
        }

        /** Represents a SceneBody. */
        class SceneBody implements ISceneBody {

            /**
             * Constructs a new SceneBody.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.ISceneBody);

            /** SceneBody shadows. */
            public shadows: zed.scene.IShadow[];

            /** SceneBody quads. */
            public quads: zed.scene.IQuad[];

            /** SceneBody underlines. */
            public underlines: zed.scene.IUnderline[];

            /** SceneBody monochromeSprites. */
            public monochromeSprites: zed.scene.IMonochromeSprite[];

            /** SceneBody subpixelSprites. */
            public subpixelSprites: zed.scene.ISubpixelSprite[];

            /** SceneBody polychromeSprites. */
            public polychromeSprites: zed.scene.IPolychromeSprite[];

            /** SceneBody paths. */
            public paths: zed.scene.IPath[];

            /**
             * Creates a new SceneBody instance using the specified properties.
             * @param [properties] Properties to set
             * @returns SceneBody instance
             */
            public static create(properties?: zed.scene.ISceneBody): zed.scene.SceneBody;

            /**
             * Encodes the specified SceneBody message. Does not implicitly {@link zed.scene.SceneBody.verify|verify} messages.
             * @param message SceneBody message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.ISceneBody, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified SceneBody message, length delimited. Does not implicitly {@link zed.scene.SceneBody.verify|verify} messages.
             * @param message SceneBody message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.ISceneBody, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a SceneBody message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns SceneBody
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.SceneBody;

            /**
             * Decodes a SceneBody message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns SceneBody
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.SceneBody;

            /**
             * Verifies a SceneBody message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a SceneBody message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns SceneBody
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.SceneBody;

            /**
             * Creates a plain object from a SceneBody message. Also converts values to other types if specified.
             * @param message SceneBody
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.SceneBody, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this SceneBody to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for SceneBody
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an AtlasEntry. */
        interface IAtlasEntry {

            /** AtlasEntry textureId */
            textureId?: (zed.scene.IAtlasTextureId|null);

            /** AtlasEntry bounds */
            bounds?: (zed.scene.IAtlasBounds|null);

            /** AtlasEntry format */
            format?: (number|null);

            /** AtlasEntry pixelData */
            pixelData?: (Uint8Array|null);
        }

        /** Represents an AtlasEntry. */
        class AtlasEntry implements IAtlasEntry {

            /**
             * Constructs a new AtlasEntry.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IAtlasEntry);

            /** AtlasEntry textureId. */
            public textureId?: (zed.scene.IAtlasTextureId|null);

            /** AtlasEntry bounds. */
            public bounds?: (zed.scene.IAtlasBounds|null);

            /** AtlasEntry format. */
            public format: number;

            /** AtlasEntry pixelData. */
            public pixelData: Uint8Array;

            /**
             * Creates a new AtlasEntry instance using the specified properties.
             * @param [properties] Properties to set
             * @returns AtlasEntry instance
             */
            public static create(properties?: zed.scene.IAtlasEntry): zed.scene.AtlasEntry;

            /**
             * Encodes the specified AtlasEntry message. Does not implicitly {@link zed.scene.AtlasEntry.verify|verify} messages.
             * @param message AtlasEntry message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IAtlasEntry, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified AtlasEntry message, length delimited. Does not implicitly {@link zed.scene.AtlasEntry.verify|verify} messages.
             * @param message AtlasEntry message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IAtlasEntry, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an AtlasEntry message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns AtlasEntry
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.AtlasEntry;

            /**
             * Decodes an AtlasEntry message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns AtlasEntry
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.AtlasEntry;

            /**
             * Verifies an AtlasEntry message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an AtlasEntry message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns AtlasEntry
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.AtlasEntry;

            /**
             * Creates a plain object from an AtlasEntry message. Also converts values to other types if specified.
             * @param message AtlasEntry
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.AtlasEntry, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this AtlasEntry to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for AtlasEntry
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Point. */
        interface IPoint {

            /** Point x */
            x?: (number|null);

            /** Point y */
            y?: (number|null);
        }

        /** Represents a Point. */
        class Point implements IPoint {

            /**
             * Constructs a new Point.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IPoint);

            /** Point x. */
            public x: number;

            /** Point y. */
            public y: number;

            /**
             * Creates a new Point instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Point instance
             */
            public static create(properties?: zed.scene.IPoint): zed.scene.Point;

            /**
             * Encodes the specified Point message. Does not implicitly {@link zed.scene.Point.verify|verify} messages.
             * @param message Point message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IPoint, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Point message, length delimited. Does not implicitly {@link zed.scene.Point.verify|verify} messages.
             * @param message Point message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IPoint, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Point message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Point
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Point;

            /**
             * Decodes a Point message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Point
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Point;

            /**
             * Verifies a Point message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Point message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Point
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Point;

            /**
             * Creates a plain object from a Point message. Also converts values to other types if specified.
             * @param message Point
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Point, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Point to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Point
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Size. */
        interface ISize {

            /** Size width */
            width?: (number|null);

            /** Size height */
            height?: (number|null);
        }

        /** Represents a Size. */
        class Size implements ISize {

            /**
             * Constructs a new Size.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.ISize);

            /** Size width. */
            public width: number;

            /** Size height. */
            public height: number;

            /**
             * Creates a new Size instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Size instance
             */
            public static create(properties?: zed.scene.ISize): zed.scene.Size;

            /**
             * Encodes the specified Size message. Does not implicitly {@link zed.scene.Size.verify|verify} messages.
             * @param message Size message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.ISize, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Size message, length delimited. Does not implicitly {@link zed.scene.Size.verify|verify} messages.
             * @param message Size message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.ISize, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Size message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Size
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Size;

            /**
             * Decodes a Size message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Size
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Size;

            /**
             * Verifies a Size message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Size message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Size
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Size;

            /**
             * Creates a plain object from a Size message. Also converts values to other types if specified.
             * @param message Size
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Size, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Size to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Size
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Bounds. */
        interface IBounds {

            /** Bounds origin */
            origin?: (zed.scene.IPoint|null);

            /** Bounds size */
            size?: (zed.scene.ISize|null);
        }

        /** Represents a Bounds. */
        class Bounds implements IBounds {

            /**
             * Constructs a new Bounds.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IBounds);

            /** Bounds origin. */
            public origin?: (zed.scene.IPoint|null);

            /** Bounds size. */
            public size?: (zed.scene.ISize|null);

            /**
             * Creates a new Bounds instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Bounds instance
             */
            public static create(properties?: zed.scene.IBounds): zed.scene.Bounds;

            /**
             * Encodes the specified Bounds message. Does not implicitly {@link zed.scene.Bounds.verify|verify} messages.
             * @param message Bounds message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IBounds, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Bounds message, length delimited. Does not implicitly {@link zed.scene.Bounds.verify|verify} messages.
             * @param message Bounds message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IBounds, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Bounds message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Bounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Bounds;

            /**
             * Decodes a Bounds message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Bounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Bounds;

            /**
             * Verifies a Bounds message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Bounds message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Bounds
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Bounds;

            /**
             * Creates a plain object from a Bounds message. Also converts values to other types if specified.
             * @param message Bounds
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Bounds, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Bounds to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Bounds
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a ContentMask. */
        interface IContentMask {

            /** ContentMask bounds */
            bounds?: (zed.scene.IBounds|null);
        }

        /** Represents a ContentMask. */
        class ContentMask implements IContentMask {

            /**
             * Constructs a new ContentMask.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IContentMask);

            /** ContentMask bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /**
             * Creates a new ContentMask instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ContentMask instance
             */
            public static create(properties?: zed.scene.IContentMask): zed.scene.ContentMask;

            /**
             * Encodes the specified ContentMask message. Does not implicitly {@link zed.scene.ContentMask.verify|verify} messages.
             * @param message ContentMask message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IContentMask, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ContentMask message, length delimited. Does not implicitly {@link zed.scene.ContentMask.verify|verify} messages.
             * @param message ContentMask message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IContentMask, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ContentMask message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ContentMask
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.ContentMask;

            /**
             * Decodes a ContentMask message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ContentMask
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.ContentMask;

            /**
             * Verifies a ContentMask message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ContentMask message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ContentMask
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.ContentMask;

            /**
             * Creates a plain object from a ContentMask message. Also converts values to other types if specified.
             * @param message ContentMask
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.ContentMask, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ContentMask to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for ContentMask
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Corners. */
        interface ICorners {

            /** Corners topLeft */
            topLeft?: (number|null);

            /** Corners topRight */
            topRight?: (number|null);

            /** Corners bottomRight */
            bottomRight?: (number|null);

            /** Corners bottomLeft */
            bottomLeft?: (number|null);
        }

        /** Represents a Corners. */
        class Corners implements ICorners {

            /**
             * Constructs a new Corners.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.ICorners);

            /** Corners topLeft. */
            public topLeft: number;

            /** Corners topRight. */
            public topRight: number;

            /** Corners bottomRight. */
            public bottomRight: number;

            /** Corners bottomLeft. */
            public bottomLeft: number;

            /**
             * Creates a new Corners instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Corners instance
             */
            public static create(properties?: zed.scene.ICorners): zed.scene.Corners;

            /**
             * Encodes the specified Corners message. Does not implicitly {@link zed.scene.Corners.verify|verify} messages.
             * @param message Corners message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.ICorners, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Corners message, length delimited. Does not implicitly {@link zed.scene.Corners.verify|verify} messages.
             * @param message Corners message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.ICorners, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Corners message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Corners
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Corners;

            /**
             * Decodes a Corners message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Corners
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Corners;

            /**
             * Verifies a Corners message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Corners message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Corners
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Corners;

            /**
             * Creates a plain object from a Corners message. Also converts values to other types if specified.
             * @param message Corners
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Corners, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Corners to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Corners
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an Edges. */
        interface IEdges {

            /** Edges top */
            top?: (number|null);

            /** Edges right */
            right?: (number|null);

            /** Edges bottom */
            bottom?: (number|null);

            /** Edges left */
            left?: (number|null);
        }

        /** Represents an Edges. */
        class Edges implements IEdges {

            /**
             * Constructs a new Edges.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IEdges);

            /** Edges top. */
            public top: number;

            /** Edges right. */
            public right: number;

            /** Edges bottom. */
            public bottom: number;

            /** Edges left. */
            public left: number;

            /**
             * Creates a new Edges instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Edges instance
             */
            public static create(properties?: zed.scene.IEdges): zed.scene.Edges;

            /**
             * Encodes the specified Edges message. Does not implicitly {@link zed.scene.Edges.verify|verify} messages.
             * @param message Edges message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IEdges, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Edges message, length delimited. Does not implicitly {@link zed.scene.Edges.verify|verify} messages.
             * @param message Edges message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IEdges, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Edges message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Edges
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Edges;

            /**
             * Decodes an Edges message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Edges
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Edges;

            /**
             * Verifies an Edges message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Edges message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Edges
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Edges;

            /**
             * Creates a plain object from an Edges message. Also converts values to other types if specified.
             * @param message Edges
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Edges, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Edges to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Edges
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Hsla. */
        interface IHsla {

            /** Hsla h */
            h?: (number|null);

            /** Hsla s */
            s?: (number|null);

            /** Hsla l */
            l?: (number|null);

            /** Hsla a */
            a?: (number|null);
        }

        /** Represents a Hsla. */
        class Hsla implements IHsla {

            /**
             * Constructs a new Hsla.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IHsla);

            /** Hsla h. */
            public h: number;

            /** Hsla s. */
            public s: number;

            /** Hsla l. */
            public l: number;

            /** Hsla a. */
            public a: number;

            /**
             * Creates a new Hsla instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Hsla instance
             */
            public static create(properties?: zed.scene.IHsla): zed.scene.Hsla;

            /**
             * Encodes the specified Hsla message. Does not implicitly {@link zed.scene.Hsla.verify|verify} messages.
             * @param message Hsla message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IHsla, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Hsla message, length delimited. Does not implicitly {@link zed.scene.Hsla.verify|verify} messages.
             * @param message Hsla message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IHsla, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Hsla message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Hsla
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Hsla;

            /**
             * Decodes a Hsla message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Hsla
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Hsla;

            /**
             * Verifies a Hsla message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Hsla message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Hsla
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Hsla;

            /**
             * Creates a plain object from a Hsla message. Also converts values to other types if specified.
             * @param message Hsla
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Hsla, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Hsla to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Hsla
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a LinearColorStop. */
        interface ILinearColorStop {

            /** LinearColorStop color */
            color?: (zed.scene.IHsla|null);

            /** LinearColorStop percentage */
            percentage?: (number|null);
        }

        /** Represents a LinearColorStop. */
        class LinearColorStop implements ILinearColorStop {

            /**
             * Constructs a new LinearColorStop.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.ILinearColorStop);

            /** LinearColorStop color. */
            public color?: (zed.scene.IHsla|null);

            /** LinearColorStop percentage. */
            public percentage: number;

            /**
             * Creates a new LinearColorStop instance using the specified properties.
             * @param [properties] Properties to set
             * @returns LinearColorStop instance
             */
            public static create(properties?: zed.scene.ILinearColorStop): zed.scene.LinearColorStop;

            /**
             * Encodes the specified LinearColorStop message. Does not implicitly {@link zed.scene.LinearColorStop.verify|verify} messages.
             * @param message LinearColorStop message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.ILinearColorStop, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified LinearColorStop message, length delimited. Does not implicitly {@link zed.scene.LinearColorStop.verify|verify} messages.
             * @param message LinearColorStop message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.ILinearColorStop, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a LinearColorStop message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns LinearColorStop
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.LinearColorStop;

            /**
             * Decodes a LinearColorStop message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns LinearColorStop
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.LinearColorStop;

            /**
             * Verifies a LinearColorStop message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a LinearColorStop message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns LinearColorStop
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.LinearColorStop;

            /**
             * Creates a plain object from a LinearColorStop message. Also converts values to other types if specified.
             * @param message LinearColorStop
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.LinearColorStop, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this LinearColorStop to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for LinearColorStop
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Background. */
        interface IBackground {

            /** Background tag */
            tag?: (number|null);

            /** Background colorSpace */
            colorSpace?: (number|null);

            /** Background solid */
            solid?: (zed.scene.IHsla|null);

            /** Background gradientAngleOrPatternHeight */
            gradientAngleOrPatternHeight?: (number|null);

            /** Background colors */
            colors?: (zed.scene.ILinearColorStop[]|null);
        }

        /** Represents a Background. */
        class Background implements IBackground {

            /**
             * Constructs a new Background.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IBackground);

            /** Background tag. */
            public tag: number;

            /** Background colorSpace. */
            public colorSpace: number;

            /** Background solid. */
            public solid?: (zed.scene.IHsla|null);

            /** Background gradientAngleOrPatternHeight. */
            public gradientAngleOrPatternHeight: number;

            /** Background colors. */
            public colors: zed.scene.ILinearColorStop[];

            /**
             * Creates a new Background instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Background instance
             */
            public static create(properties?: zed.scene.IBackground): zed.scene.Background;

            /**
             * Encodes the specified Background message. Does not implicitly {@link zed.scene.Background.verify|verify} messages.
             * @param message Background message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IBackground, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Background message, length delimited. Does not implicitly {@link zed.scene.Background.verify|verify} messages.
             * @param message Background message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IBackground, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Background message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Background
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Background;

            /**
             * Decodes a Background message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Background
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Background;

            /**
             * Verifies a Background message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Background message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Background
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Background;

            /**
             * Creates a plain object from a Background message. Also converts values to other types if specified.
             * @param message Background
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Background, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Background to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Background
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an AtlasTextureId. */
        interface IAtlasTextureId {

            /** AtlasTextureId index */
            index?: (number|null);

            /** AtlasTextureId kind */
            kind?: (number|null);
        }

        /** Represents an AtlasTextureId. */
        class AtlasTextureId implements IAtlasTextureId {

            /**
             * Constructs a new AtlasTextureId.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IAtlasTextureId);

            /** AtlasTextureId index. */
            public index: number;

            /** AtlasTextureId kind. */
            public kind: number;

            /**
             * Creates a new AtlasTextureId instance using the specified properties.
             * @param [properties] Properties to set
             * @returns AtlasTextureId instance
             */
            public static create(properties?: zed.scene.IAtlasTextureId): zed.scene.AtlasTextureId;

            /**
             * Encodes the specified AtlasTextureId message. Does not implicitly {@link zed.scene.AtlasTextureId.verify|verify} messages.
             * @param message AtlasTextureId message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IAtlasTextureId, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified AtlasTextureId message, length delimited. Does not implicitly {@link zed.scene.AtlasTextureId.verify|verify} messages.
             * @param message AtlasTextureId message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IAtlasTextureId, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an AtlasTextureId message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns AtlasTextureId
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.AtlasTextureId;

            /**
             * Decodes an AtlasTextureId message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns AtlasTextureId
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.AtlasTextureId;

            /**
             * Verifies an AtlasTextureId message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an AtlasTextureId message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns AtlasTextureId
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.AtlasTextureId;

            /**
             * Creates a plain object from an AtlasTextureId message. Also converts values to other types if specified.
             * @param message AtlasTextureId
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.AtlasTextureId, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this AtlasTextureId to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for AtlasTextureId
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an AtlasBounds. */
        interface IAtlasBounds {

            /** AtlasBounds originX */
            originX?: (number|null);

            /** AtlasBounds originY */
            originY?: (number|null);

            /** AtlasBounds width */
            width?: (number|null);

            /** AtlasBounds height */
            height?: (number|null);
        }

        /** Represents an AtlasBounds. */
        class AtlasBounds implements IAtlasBounds {

            /**
             * Constructs a new AtlasBounds.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IAtlasBounds);

            /** AtlasBounds originX. */
            public originX: number;

            /** AtlasBounds originY. */
            public originY: number;

            /** AtlasBounds width. */
            public width: number;

            /** AtlasBounds height. */
            public height: number;

            /**
             * Creates a new AtlasBounds instance using the specified properties.
             * @param [properties] Properties to set
             * @returns AtlasBounds instance
             */
            public static create(properties?: zed.scene.IAtlasBounds): zed.scene.AtlasBounds;

            /**
             * Encodes the specified AtlasBounds message. Does not implicitly {@link zed.scene.AtlasBounds.verify|verify} messages.
             * @param message AtlasBounds message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IAtlasBounds, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified AtlasBounds message, length delimited. Does not implicitly {@link zed.scene.AtlasBounds.verify|verify} messages.
             * @param message AtlasBounds message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IAtlasBounds, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an AtlasBounds message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns AtlasBounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.AtlasBounds;

            /**
             * Decodes an AtlasBounds message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns AtlasBounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.AtlasBounds;

            /**
             * Verifies an AtlasBounds message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an AtlasBounds message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns AtlasBounds
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.AtlasBounds;

            /**
             * Creates a plain object from an AtlasBounds message. Also converts values to other types if specified.
             * @param message AtlasBounds
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.AtlasBounds, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this AtlasBounds to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for AtlasBounds
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an AtlasTile. */
        interface IAtlasTile {

            /** AtlasTile textureId */
            textureId?: (zed.scene.IAtlasTextureId|null);

            /** AtlasTile tileId */
            tileId?: (number|null);

            /** AtlasTile padding */
            padding?: (number|null);

            /** AtlasTile bounds */
            bounds?: (zed.scene.IAtlasBounds|null);
        }

        /** Represents an AtlasTile. */
        class AtlasTile implements IAtlasTile {

            /**
             * Constructs a new AtlasTile.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IAtlasTile);

            /** AtlasTile textureId. */
            public textureId?: (zed.scene.IAtlasTextureId|null);

            /** AtlasTile tileId. */
            public tileId: number;

            /** AtlasTile padding. */
            public padding: number;

            /** AtlasTile bounds. */
            public bounds?: (zed.scene.IAtlasBounds|null);

            /**
             * Creates a new AtlasTile instance using the specified properties.
             * @param [properties] Properties to set
             * @returns AtlasTile instance
             */
            public static create(properties?: zed.scene.IAtlasTile): zed.scene.AtlasTile;

            /**
             * Encodes the specified AtlasTile message. Does not implicitly {@link zed.scene.AtlasTile.verify|verify} messages.
             * @param message AtlasTile message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IAtlasTile, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified AtlasTile message, length delimited. Does not implicitly {@link zed.scene.AtlasTile.verify|verify} messages.
             * @param message AtlasTile message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IAtlasTile, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an AtlasTile message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns AtlasTile
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.AtlasTile;

            /**
             * Decodes an AtlasTile message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns AtlasTile
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.AtlasTile;

            /**
             * Verifies an AtlasTile message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an AtlasTile message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns AtlasTile
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.AtlasTile;

            /**
             * Creates a plain object from an AtlasTile message. Also converts values to other types if specified.
             * @param message AtlasTile
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.AtlasTile, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this AtlasTile to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for AtlasTile
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a TransformationMatrix. */
        interface ITransformationMatrix {

            /** TransformationMatrix r00 */
            r00?: (number|null);

            /** TransformationMatrix r01 */
            r01?: (number|null);

            /** TransformationMatrix r10 */
            r10?: (number|null);

            /** TransformationMatrix r11 */
            r11?: (number|null);

            /** TransformationMatrix tx */
            tx?: (number|null);

            /** TransformationMatrix ty */
            ty?: (number|null);
        }

        /** Represents a TransformationMatrix. */
        class TransformationMatrix implements ITransformationMatrix {

            /**
             * Constructs a new TransformationMatrix.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.ITransformationMatrix);

            /** TransformationMatrix r00. */
            public r00: number;

            /** TransformationMatrix r01. */
            public r01: number;

            /** TransformationMatrix r10. */
            public r10: number;

            /** TransformationMatrix r11. */
            public r11: number;

            /** TransformationMatrix tx. */
            public tx: number;

            /** TransformationMatrix ty. */
            public ty: number;

            /**
             * Creates a new TransformationMatrix instance using the specified properties.
             * @param [properties] Properties to set
             * @returns TransformationMatrix instance
             */
            public static create(properties?: zed.scene.ITransformationMatrix): zed.scene.TransformationMatrix;

            /**
             * Encodes the specified TransformationMatrix message. Does not implicitly {@link zed.scene.TransformationMatrix.verify|verify} messages.
             * @param message TransformationMatrix message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.ITransformationMatrix, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified TransformationMatrix message, length delimited. Does not implicitly {@link zed.scene.TransformationMatrix.verify|verify} messages.
             * @param message TransformationMatrix message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.ITransformationMatrix, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a TransformationMatrix message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns TransformationMatrix
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.TransformationMatrix;

            /**
             * Decodes a TransformationMatrix message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns TransformationMatrix
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.TransformationMatrix;

            /**
             * Verifies a TransformationMatrix message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a TransformationMatrix message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns TransformationMatrix
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.TransformationMatrix;

            /**
             * Creates a plain object from a TransformationMatrix message. Also converts values to other types if specified.
             * @param message TransformationMatrix
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.TransformationMatrix, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this TransformationMatrix to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for TransformationMatrix
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Shadow. */
        interface IShadow {

            /** Shadow order */
            order?: (number|null);

            /** Shadow blurRadius */
            blurRadius?: (number|null);

            /** Shadow bounds */
            bounds?: (zed.scene.IBounds|null);

            /** Shadow cornerRadii */
            cornerRadii?: (zed.scene.ICorners|null);

            /** Shadow contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** Shadow color */
            color?: (zed.scene.IHsla|null);
        }

        /** Represents a Shadow. */
        class Shadow implements IShadow {

            /**
             * Constructs a new Shadow.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IShadow);

            /** Shadow order. */
            public order: number;

            /** Shadow blurRadius. */
            public blurRadius: number;

            /** Shadow bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** Shadow cornerRadii. */
            public cornerRadii?: (zed.scene.ICorners|null);

            /** Shadow contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** Shadow color. */
            public color?: (zed.scene.IHsla|null);

            /**
             * Creates a new Shadow instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Shadow instance
             */
            public static create(properties?: zed.scene.IShadow): zed.scene.Shadow;

            /**
             * Encodes the specified Shadow message. Does not implicitly {@link zed.scene.Shadow.verify|verify} messages.
             * @param message Shadow message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IShadow, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Shadow message, length delimited. Does not implicitly {@link zed.scene.Shadow.verify|verify} messages.
             * @param message Shadow message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IShadow, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Shadow message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Shadow
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Shadow;

            /**
             * Decodes a Shadow message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Shadow
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Shadow;

            /**
             * Verifies a Shadow message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Shadow message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Shadow
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Shadow;

            /**
             * Creates a plain object from a Shadow message. Also converts values to other types if specified.
             * @param message Shadow
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Shadow, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Shadow to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Shadow
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Quad. */
        interface IQuad {

            /** Quad order */
            order?: (number|null);

            /** Quad borderStyle */
            borderStyle?: (number|null);

            /** Quad bounds */
            bounds?: (zed.scene.IBounds|null);

            /** Quad contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** Quad background */
            background?: (zed.scene.IBackground|null);

            /** Quad borderColor */
            borderColor?: (zed.scene.IHsla|null);

            /** Quad cornerRadii */
            cornerRadii?: (zed.scene.ICorners|null);

            /** Quad borderWidths */
            borderWidths?: (zed.scene.IEdges|null);
        }

        /** Represents a Quad. */
        class Quad implements IQuad {

            /**
             * Constructs a new Quad.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IQuad);

            /** Quad order. */
            public order: number;

            /** Quad borderStyle. */
            public borderStyle: number;

            /** Quad bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** Quad contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** Quad background. */
            public background?: (zed.scene.IBackground|null);

            /** Quad borderColor. */
            public borderColor?: (zed.scene.IHsla|null);

            /** Quad cornerRadii. */
            public cornerRadii?: (zed.scene.ICorners|null);

            /** Quad borderWidths. */
            public borderWidths?: (zed.scene.IEdges|null);

            /**
             * Creates a new Quad instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Quad instance
             */
            public static create(properties?: zed.scene.IQuad): zed.scene.Quad;

            /**
             * Encodes the specified Quad message. Does not implicitly {@link zed.scene.Quad.verify|verify} messages.
             * @param message Quad message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IQuad, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Quad message, length delimited. Does not implicitly {@link zed.scene.Quad.verify|verify} messages.
             * @param message Quad message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IQuad, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Quad message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Quad
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Quad;

            /**
             * Decodes a Quad message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Quad
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Quad;

            /**
             * Verifies a Quad message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Quad message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Quad
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Quad;

            /**
             * Creates a plain object from a Quad message. Also converts values to other types if specified.
             * @param message Quad
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Quad, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Quad to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Quad
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an Underline. */
        interface IUnderline {

            /** Underline order */
            order?: (number|null);

            /** Underline bounds */
            bounds?: (zed.scene.IBounds|null);

            /** Underline contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** Underline color */
            color?: (zed.scene.IHsla|null);

            /** Underline thickness */
            thickness?: (number|null);

            /** Underline wavy */
            wavy?: (boolean|null);
        }

        /** Represents an Underline. */
        class Underline implements IUnderline {

            /**
             * Constructs a new Underline.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IUnderline);

            /** Underline order. */
            public order: number;

            /** Underline bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** Underline contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** Underline color. */
            public color?: (zed.scene.IHsla|null);

            /** Underline thickness. */
            public thickness: number;

            /** Underline wavy. */
            public wavy: boolean;

            /**
             * Creates a new Underline instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Underline instance
             */
            public static create(properties?: zed.scene.IUnderline): zed.scene.Underline;

            /**
             * Encodes the specified Underline message. Does not implicitly {@link zed.scene.Underline.verify|verify} messages.
             * @param message Underline message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IUnderline, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Underline message, length delimited. Does not implicitly {@link zed.scene.Underline.verify|verify} messages.
             * @param message Underline message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IUnderline, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Underline message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Underline
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Underline;

            /**
             * Decodes an Underline message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Underline
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Underline;

            /**
             * Verifies an Underline message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Underline message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Underline
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Underline;

            /**
             * Creates a plain object from an Underline message. Also converts values to other types if specified.
             * @param message Underline
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Underline, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Underline to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Underline
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a MonochromeSprite. */
        interface IMonochromeSprite {

            /** MonochromeSprite order */
            order?: (number|null);

            /** MonochromeSprite bounds */
            bounds?: (zed.scene.IBounds|null);

            /** MonochromeSprite contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** MonochromeSprite color */
            color?: (zed.scene.IHsla|null);

            /** MonochromeSprite tile */
            tile?: (zed.scene.IAtlasTile|null);

            /** MonochromeSprite transformation */
            transformation?: (zed.scene.ITransformationMatrix|null);
        }

        /** Represents a MonochromeSprite. */
        class MonochromeSprite implements IMonochromeSprite {

            /**
             * Constructs a new MonochromeSprite.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IMonochromeSprite);

            /** MonochromeSprite order. */
            public order: number;

            /** MonochromeSprite bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** MonochromeSprite contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** MonochromeSprite color. */
            public color?: (zed.scene.IHsla|null);

            /** MonochromeSprite tile. */
            public tile?: (zed.scene.IAtlasTile|null);

            /** MonochromeSprite transformation. */
            public transformation?: (zed.scene.ITransformationMatrix|null);

            /**
             * Creates a new MonochromeSprite instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MonochromeSprite instance
             */
            public static create(properties?: zed.scene.IMonochromeSprite): zed.scene.MonochromeSprite;

            /**
             * Encodes the specified MonochromeSprite message. Does not implicitly {@link zed.scene.MonochromeSprite.verify|verify} messages.
             * @param message MonochromeSprite message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IMonochromeSprite, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MonochromeSprite message, length delimited. Does not implicitly {@link zed.scene.MonochromeSprite.verify|verify} messages.
             * @param message MonochromeSprite message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IMonochromeSprite, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MonochromeSprite message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MonochromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.MonochromeSprite;

            /**
             * Decodes a MonochromeSprite message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MonochromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.MonochromeSprite;

            /**
             * Verifies a MonochromeSprite message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MonochromeSprite message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MonochromeSprite
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.MonochromeSprite;

            /**
             * Creates a plain object from a MonochromeSprite message. Also converts values to other types if specified.
             * @param message MonochromeSprite
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.MonochromeSprite, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MonochromeSprite to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for MonochromeSprite
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a SubpixelSprite. */
        interface ISubpixelSprite {

            /** SubpixelSprite order */
            order?: (number|null);

            /** SubpixelSprite bounds */
            bounds?: (zed.scene.IBounds|null);

            /** SubpixelSprite contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** SubpixelSprite color */
            color?: (zed.scene.IHsla|null);

            /** SubpixelSprite tile */
            tile?: (zed.scene.IAtlasTile|null);

            /** SubpixelSprite transformation */
            transformation?: (zed.scene.ITransformationMatrix|null);
        }

        /** Represents a SubpixelSprite. */
        class SubpixelSprite implements ISubpixelSprite {

            /**
             * Constructs a new SubpixelSprite.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.ISubpixelSprite);

            /** SubpixelSprite order. */
            public order: number;

            /** SubpixelSprite bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** SubpixelSprite contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** SubpixelSprite color. */
            public color?: (zed.scene.IHsla|null);

            /** SubpixelSprite tile. */
            public tile?: (zed.scene.IAtlasTile|null);

            /** SubpixelSprite transformation. */
            public transformation?: (zed.scene.ITransformationMatrix|null);

            /**
             * Creates a new SubpixelSprite instance using the specified properties.
             * @param [properties] Properties to set
             * @returns SubpixelSprite instance
             */
            public static create(properties?: zed.scene.ISubpixelSprite): zed.scene.SubpixelSprite;

            /**
             * Encodes the specified SubpixelSprite message. Does not implicitly {@link zed.scene.SubpixelSprite.verify|verify} messages.
             * @param message SubpixelSprite message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.ISubpixelSprite, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified SubpixelSprite message, length delimited. Does not implicitly {@link zed.scene.SubpixelSprite.verify|verify} messages.
             * @param message SubpixelSprite message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.ISubpixelSprite, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a SubpixelSprite message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns SubpixelSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.SubpixelSprite;

            /**
             * Decodes a SubpixelSprite message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns SubpixelSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.SubpixelSprite;

            /**
             * Verifies a SubpixelSprite message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a SubpixelSprite message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns SubpixelSprite
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.SubpixelSprite;

            /**
             * Creates a plain object from a SubpixelSprite message. Also converts values to other types if specified.
             * @param message SubpixelSprite
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.SubpixelSprite, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this SubpixelSprite to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for SubpixelSprite
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a PolychromeSprite. */
        interface IPolychromeSprite {

            /** PolychromeSprite order */
            order?: (number|null);

            /** PolychromeSprite grayscale */
            grayscale?: (boolean|null);

            /** PolychromeSprite opacity */
            opacity?: (number|null);

            /** PolychromeSprite bounds */
            bounds?: (zed.scene.IBounds|null);

            /** PolychromeSprite contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** PolychromeSprite cornerRadii */
            cornerRadii?: (zed.scene.ICorners|null);

            /** PolychromeSprite tile */
            tile?: (zed.scene.IAtlasTile|null);
        }

        /** Represents a PolychromeSprite. */
        class PolychromeSprite implements IPolychromeSprite {

            /**
             * Constructs a new PolychromeSprite.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IPolychromeSprite);

            /** PolychromeSprite order. */
            public order: number;

            /** PolychromeSprite grayscale. */
            public grayscale: boolean;

            /** PolychromeSprite opacity. */
            public opacity: number;

            /** PolychromeSprite bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** PolychromeSprite contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** PolychromeSprite cornerRadii. */
            public cornerRadii?: (zed.scene.ICorners|null);

            /** PolychromeSprite tile. */
            public tile?: (zed.scene.IAtlasTile|null);

            /**
             * Creates a new PolychromeSprite instance using the specified properties.
             * @param [properties] Properties to set
             * @returns PolychromeSprite instance
             */
            public static create(properties?: zed.scene.IPolychromeSprite): zed.scene.PolychromeSprite;

            /**
             * Encodes the specified PolychromeSprite message. Does not implicitly {@link zed.scene.PolychromeSprite.verify|verify} messages.
             * @param message PolychromeSprite message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IPolychromeSprite, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified PolychromeSprite message, length delimited. Does not implicitly {@link zed.scene.PolychromeSprite.verify|verify} messages.
             * @param message PolychromeSprite message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IPolychromeSprite, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a PolychromeSprite message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns PolychromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.PolychromeSprite;

            /**
             * Decodes a PolychromeSprite message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns PolychromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.PolychromeSprite;

            /**
             * Verifies a PolychromeSprite message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a PolychromeSprite message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns PolychromeSprite
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.PolychromeSprite;

            /**
             * Creates a plain object from a PolychromeSprite message. Also converts values to other types if specified.
             * @param message PolychromeSprite
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.PolychromeSprite, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this PolychromeSprite to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for PolychromeSprite
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a PathVertex. */
        interface IPathVertex {

            /** PathVertex xyPosition */
            xyPosition?: (zed.scene.IPoint|null);

            /** PathVertex stPosition */
            stPosition?: (zed.scene.IPoint|null);

            /** PathVertex contentMask */
            contentMask?: (zed.scene.IContentMask|null);
        }

        /** Represents a PathVertex. */
        class PathVertex implements IPathVertex {

            /**
             * Constructs a new PathVertex.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IPathVertex);

            /** PathVertex xyPosition. */
            public xyPosition?: (zed.scene.IPoint|null);

            /** PathVertex stPosition. */
            public stPosition?: (zed.scene.IPoint|null);

            /** PathVertex contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /**
             * Creates a new PathVertex instance using the specified properties.
             * @param [properties] Properties to set
             * @returns PathVertex instance
             */
            public static create(properties?: zed.scene.IPathVertex): zed.scene.PathVertex;

            /**
             * Encodes the specified PathVertex message. Does not implicitly {@link zed.scene.PathVertex.verify|verify} messages.
             * @param message PathVertex message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IPathVertex, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified PathVertex message, length delimited. Does not implicitly {@link zed.scene.PathVertex.verify|verify} messages.
             * @param message PathVertex message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IPathVertex, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a PathVertex message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns PathVertex
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.PathVertex;

            /**
             * Decodes a PathVertex message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns PathVertex
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.PathVertex;

            /**
             * Verifies a PathVertex message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a PathVertex message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns PathVertex
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.PathVertex;

            /**
             * Creates a plain object from a PathVertex message. Also converts values to other types if specified.
             * @param message PathVertex
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.PathVertex, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this PathVertex to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for PathVertex
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Path. */
        interface IPath {

            /** Path order */
            order?: (number|null);

            /** Path bounds */
            bounds?: (zed.scene.IBounds|null);

            /** Path contentMask */
            contentMask?: (zed.scene.IContentMask|null);

            /** Path color */
            color?: (zed.scene.IBackground|null);

            /** Path vertices */
            vertices?: (zed.scene.IPathVertex[]|null);
        }

        /** Represents a Path. */
        class Path implements IPath {

            /**
             * Constructs a new Path.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IPath);

            /** Path order. */
            public order: number;

            /** Path bounds. */
            public bounds?: (zed.scene.IBounds|null);

            /** Path contentMask. */
            public contentMask?: (zed.scene.IContentMask|null);

            /** Path color. */
            public color?: (zed.scene.IBackground|null);

            /** Path vertices. */
            public vertices: zed.scene.IPathVertex[];

            /**
             * Creates a new Path instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Path instance
             */
            public static create(properties?: zed.scene.IPath): zed.scene.Path;

            /**
             * Encodes the specified Path message. Does not implicitly {@link zed.scene.Path.verify|verify} messages.
             * @param message Path message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IPath, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Path message, length delimited. Does not implicitly {@link zed.scene.Path.verify|verify} messages.
             * @param message Path message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IPath, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Path message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Path
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Path;

            /**
             * Decodes a Path message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Path
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Path;

            /**
             * Verifies a Path message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Path message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Path
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Path;

            /**
             * Creates a plain object from a Path message. Also converts values to other types if specified.
             * @param message Path
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Path, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Path to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Path
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of an InputMessage. */
        interface IInputMessage {

            /** InputMessage mouseMove */
            mouseMove?: (zed.scene.IMouseMoveInput|null);

            /** InputMessage mouseDown */
            mouseDown?: (zed.scene.IMouseDownInput|null);

            /** InputMessage mouseUp */
            mouseUp?: (zed.scene.IMouseUpInput|null);

            /** InputMessage scroll */
            scroll?: (zed.scene.IScrollInput|null);

            /** InputMessage keyDown */
            keyDown?: (zed.scene.IKeyDownInput|null);

            /** InputMessage keyUp */
            keyUp?: (zed.scene.IKeyUpInput|null);

            /** InputMessage resize */
            resize?: (zed.scene.IResizeInput|null);
        }

        /** Represents an InputMessage. */
        class InputMessage implements IInputMessage {

            /**
             * Constructs a new InputMessage.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IInputMessage);

            /** InputMessage mouseMove. */
            public mouseMove?: (zed.scene.IMouseMoveInput|null);

            /** InputMessage mouseDown. */
            public mouseDown?: (zed.scene.IMouseDownInput|null);

            /** InputMessage mouseUp. */
            public mouseUp?: (zed.scene.IMouseUpInput|null);

            /** InputMessage scroll. */
            public scroll?: (zed.scene.IScrollInput|null);

            /** InputMessage keyDown. */
            public keyDown?: (zed.scene.IKeyDownInput|null);

            /** InputMessage keyUp. */
            public keyUp?: (zed.scene.IKeyUpInput|null);

            /** InputMessage resize. */
            public resize?: (zed.scene.IResizeInput|null);

            /** InputMessage kind. */
            public kind?: ("mouseMove"|"mouseDown"|"mouseUp"|"scroll"|"keyDown"|"keyUp"|"resize");

            /**
             * Creates a new InputMessage instance using the specified properties.
             * @param [properties] Properties to set
             * @returns InputMessage instance
             */
            public static create(properties?: zed.scene.IInputMessage): zed.scene.InputMessage;

            /**
             * Encodes the specified InputMessage message. Does not implicitly {@link zed.scene.InputMessage.verify|verify} messages.
             * @param message InputMessage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IInputMessage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified InputMessage message, length delimited. Does not implicitly {@link zed.scene.InputMessage.verify|verify} messages.
             * @param message InputMessage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IInputMessage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an InputMessage message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns InputMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.InputMessage;

            /**
             * Decodes an InputMessage message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns InputMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.InputMessage;

            /**
             * Verifies an InputMessage message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an InputMessage message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns InputMessage
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.InputMessage;

            /**
             * Creates a plain object from an InputMessage message. Also converts values to other types if specified.
             * @param message InputMessage
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.InputMessage, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this InputMessage to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for InputMessage
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a Modifiers. */
        interface IModifiers {

            /** Modifiers control */
            control?: (boolean|null);

            /** Modifiers alt */
            alt?: (boolean|null);

            /** Modifiers shift */
            shift?: (boolean|null);

            /** Modifiers meta */
            meta?: (boolean|null);
        }

        /** Represents a Modifiers. */
        class Modifiers implements IModifiers {

            /**
             * Constructs a new Modifiers.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IModifiers);

            /** Modifiers control. */
            public control: boolean;

            /** Modifiers alt. */
            public alt: boolean;

            /** Modifiers shift. */
            public shift: boolean;

            /** Modifiers meta. */
            public meta: boolean;

            /**
             * Creates a new Modifiers instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Modifiers instance
             */
            public static create(properties?: zed.scene.IModifiers): zed.scene.Modifiers;

            /**
             * Encodes the specified Modifiers message. Does not implicitly {@link zed.scene.Modifiers.verify|verify} messages.
             * @param message Modifiers message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IModifiers, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Modifiers message, length delimited. Does not implicitly {@link zed.scene.Modifiers.verify|verify} messages.
             * @param message Modifiers message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IModifiers, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Modifiers message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Modifiers
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.Modifiers;

            /**
             * Decodes a Modifiers message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Modifiers
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.Modifiers;

            /**
             * Verifies a Modifiers message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Modifiers message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Modifiers
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.Modifiers;

            /**
             * Creates a plain object from a Modifiers message. Also converts values to other types if specified.
             * @param message Modifiers
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.Modifiers, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Modifiers to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for Modifiers
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a MouseMoveInput. */
        interface IMouseMoveInput {

            /** MouseMoveInput position */
            position?: (zed.scene.IPoint|null);

            /** MouseMoveInput modifiers */
            modifiers?: (zed.scene.IModifiers|null);
        }

        /** Represents a MouseMoveInput. */
        class MouseMoveInput implements IMouseMoveInput {

            /**
             * Constructs a new MouseMoveInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IMouseMoveInput);

            /** MouseMoveInput position. */
            public position?: (zed.scene.IPoint|null);

            /** MouseMoveInput modifiers. */
            public modifiers?: (zed.scene.IModifiers|null);

            /**
             * Creates a new MouseMoveInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MouseMoveInput instance
             */
            public static create(properties?: zed.scene.IMouseMoveInput): zed.scene.MouseMoveInput;

            /**
             * Encodes the specified MouseMoveInput message. Does not implicitly {@link zed.scene.MouseMoveInput.verify|verify} messages.
             * @param message MouseMoveInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IMouseMoveInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MouseMoveInput message, length delimited. Does not implicitly {@link zed.scene.MouseMoveInput.verify|verify} messages.
             * @param message MouseMoveInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IMouseMoveInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MouseMoveInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MouseMoveInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.MouseMoveInput;

            /**
             * Decodes a MouseMoveInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MouseMoveInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.MouseMoveInput;

            /**
             * Verifies a MouseMoveInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MouseMoveInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MouseMoveInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.MouseMoveInput;

            /**
             * Creates a plain object from a MouseMoveInput message. Also converts values to other types if specified.
             * @param message MouseMoveInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.MouseMoveInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MouseMoveInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for MouseMoveInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a MouseDownInput. */
        interface IMouseDownInput {

            /** MouseDownInput button */
            button?: (number|null);

            /** MouseDownInput position */
            position?: (zed.scene.IPoint|null);

            /** MouseDownInput clickCount */
            clickCount?: (number|null);

            /** MouseDownInput modifiers */
            modifiers?: (zed.scene.IModifiers|null);
        }

        /** Represents a MouseDownInput. */
        class MouseDownInput implements IMouseDownInput {

            /**
             * Constructs a new MouseDownInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IMouseDownInput);

            /** MouseDownInput button. */
            public button: number;

            /** MouseDownInput position. */
            public position?: (zed.scene.IPoint|null);

            /** MouseDownInput clickCount. */
            public clickCount: number;

            /** MouseDownInput modifiers. */
            public modifiers?: (zed.scene.IModifiers|null);

            /**
             * Creates a new MouseDownInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MouseDownInput instance
             */
            public static create(properties?: zed.scene.IMouseDownInput): zed.scene.MouseDownInput;

            /**
             * Encodes the specified MouseDownInput message. Does not implicitly {@link zed.scene.MouseDownInput.verify|verify} messages.
             * @param message MouseDownInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IMouseDownInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MouseDownInput message, length delimited. Does not implicitly {@link zed.scene.MouseDownInput.verify|verify} messages.
             * @param message MouseDownInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IMouseDownInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MouseDownInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MouseDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.MouseDownInput;

            /**
             * Decodes a MouseDownInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MouseDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.MouseDownInput;

            /**
             * Verifies a MouseDownInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MouseDownInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MouseDownInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.MouseDownInput;

            /**
             * Creates a plain object from a MouseDownInput message. Also converts values to other types if specified.
             * @param message MouseDownInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.MouseDownInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MouseDownInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for MouseDownInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a MouseUpInput. */
        interface IMouseUpInput {

            /** MouseUpInput button */
            button?: (number|null);

            /** MouseUpInput position */
            position?: (zed.scene.IPoint|null);

            /** MouseUpInput modifiers */
            modifiers?: (zed.scene.IModifiers|null);
        }

        /** Represents a MouseUpInput. */
        class MouseUpInput implements IMouseUpInput {

            /**
             * Constructs a new MouseUpInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IMouseUpInput);

            /** MouseUpInput button. */
            public button: number;

            /** MouseUpInput position. */
            public position?: (zed.scene.IPoint|null);

            /** MouseUpInput modifiers. */
            public modifiers?: (zed.scene.IModifiers|null);

            /**
             * Creates a new MouseUpInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MouseUpInput instance
             */
            public static create(properties?: zed.scene.IMouseUpInput): zed.scene.MouseUpInput;

            /**
             * Encodes the specified MouseUpInput message. Does not implicitly {@link zed.scene.MouseUpInput.verify|verify} messages.
             * @param message MouseUpInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IMouseUpInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MouseUpInput message, length delimited. Does not implicitly {@link zed.scene.MouseUpInput.verify|verify} messages.
             * @param message MouseUpInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IMouseUpInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MouseUpInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MouseUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.MouseUpInput;

            /**
             * Decodes a MouseUpInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MouseUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.MouseUpInput;

            /**
             * Verifies a MouseUpInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MouseUpInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MouseUpInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.MouseUpInput;

            /**
             * Creates a plain object from a MouseUpInput message. Also converts values to other types if specified.
             * @param message MouseUpInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.MouseUpInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MouseUpInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for MouseUpInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a ScrollInput. */
        interface IScrollInput {

            /** ScrollInput position */
            position?: (zed.scene.IPoint|null);

            /** ScrollInput delta */
            delta?: (zed.scene.IPoint|null);

            /** ScrollInput modifiers */
            modifiers?: (zed.scene.IModifiers|null);
        }

        /** Represents a ScrollInput. */
        class ScrollInput implements IScrollInput {

            /**
             * Constructs a new ScrollInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IScrollInput);

            /** ScrollInput position. */
            public position?: (zed.scene.IPoint|null);

            /** ScrollInput delta. */
            public delta?: (zed.scene.IPoint|null);

            /** ScrollInput modifiers. */
            public modifiers?: (zed.scene.IModifiers|null);

            /**
             * Creates a new ScrollInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ScrollInput instance
             */
            public static create(properties?: zed.scene.IScrollInput): zed.scene.ScrollInput;

            /**
             * Encodes the specified ScrollInput message. Does not implicitly {@link zed.scene.ScrollInput.verify|verify} messages.
             * @param message ScrollInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IScrollInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ScrollInput message, length delimited. Does not implicitly {@link zed.scene.ScrollInput.verify|verify} messages.
             * @param message ScrollInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IScrollInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ScrollInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ScrollInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.ScrollInput;

            /**
             * Decodes a ScrollInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ScrollInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.ScrollInput;

            /**
             * Verifies a ScrollInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ScrollInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ScrollInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.ScrollInput;

            /**
             * Creates a plain object from a ScrollInput message. Also converts values to other types if specified.
             * @param message ScrollInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.ScrollInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ScrollInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for ScrollInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a KeyDownInput. */
        interface IKeyDownInput {

            /** KeyDownInput key */
            key?: (string|null);

            /** KeyDownInput modifiers */
            modifiers?: (zed.scene.IModifiers|null);
        }

        /** Represents a KeyDownInput. */
        class KeyDownInput implements IKeyDownInput {

            /**
             * Constructs a new KeyDownInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IKeyDownInput);

            /** KeyDownInput key. */
            public key: string;

            /** KeyDownInput modifiers. */
            public modifiers?: (zed.scene.IModifiers|null);

            /**
             * Creates a new KeyDownInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns KeyDownInput instance
             */
            public static create(properties?: zed.scene.IKeyDownInput): zed.scene.KeyDownInput;

            /**
             * Encodes the specified KeyDownInput message. Does not implicitly {@link zed.scene.KeyDownInput.verify|verify} messages.
             * @param message KeyDownInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IKeyDownInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified KeyDownInput message, length delimited. Does not implicitly {@link zed.scene.KeyDownInput.verify|verify} messages.
             * @param message KeyDownInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IKeyDownInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a KeyDownInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns KeyDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.KeyDownInput;

            /**
             * Decodes a KeyDownInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns KeyDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.KeyDownInput;

            /**
             * Verifies a KeyDownInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a KeyDownInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns KeyDownInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.KeyDownInput;

            /**
             * Creates a plain object from a KeyDownInput message. Also converts values to other types if specified.
             * @param message KeyDownInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.KeyDownInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this KeyDownInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for KeyDownInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a KeyUpInput. */
        interface IKeyUpInput {

            /** KeyUpInput key */
            key?: (string|null);

            /** KeyUpInput modifiers */
            modifiers?: (zed.scene.IModifiers|null);
        }

        /** Represents a KeyUpInput. */
        class KeyUpInput implements IKeyUpInput {

            /**
             * Constructs a new KeyUpInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IKeyUpInput);

            /** KeyUpInput key. */
            public key: string;

            /** KeyUpInput modifiers. */
            public modifiers?: (zed.scene.IModifiers|null);

            /**
             * Creates a new KeyUpInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns KeyUpInput instance
             */
            public static create(properties?: zed.scene.IKeyUpInput): zed.scene.KeyUpInput;

            /**
             * Encodes the specified KeyUpInput message. Does not implicitly {@link zed.scene.KeyUpInput.verify|verify} messages.
             * @param message KeyUpInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IKeyUpInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified KeyUpInput message, length delimited. Does not implicitly {@link zed.scene.KeyUpInput.verify|verify} messages.
             * @param message KeyUpInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IKeyUpInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a KeyUpInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns KeyUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.KeyUpInput;

            /**
             * Decodes a KeyUpInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns KeyUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.KeyUpInput;

            /**
             * Verifies a KeyUpInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a KeyUpInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns KeyUpInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.KeyUpInput;

            /**
             * Creates a plain object from a KeyUpInput message. Also converts values to other types if specified.
             * @param message KeyUpInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.KeyUpInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this KeyUpInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for KeyUpInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        /** Properties of a ResizeInput. */
        interface IResizeInput {

            /** ResizeInput size */
            size?: (zed.scene.ISize|null);

            /** ResizeInput scaleFactor */
            scaleFactor?: (number|null);
        }

        /** Represents a ResizeInput. */
        class ResizeInput implements IResizeInput {

            /**
             * Constructs a new ResizeInput.
             * @param [properties] Properties to set
             */
            constructor(properties?: zed.scene.IResizeInput);

            /** ResizeInput size. */
            public size?: (zed.scene.ISize|null);

            /** ResizeInput scaleFactor. */
            public scaleFactor: number;

            /**
             * Creates a new ResizeInput instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ResizeInput instance
             */
            public static create(properties?: zed.scene.IResizeInput): zed.scene.ResizeInput;

            /**
             * Encodes the specified ResizeInput message. Does not implicitly {@link zed.scene.ResizeInput.verify|verify} messages.
             * @param message ResizeInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: zed.scene.IResizeInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ResizeInput message, length delimited. Does not implicitly {@link zed.scene.ResizeInput.verify|verify} messages.
             * @param message ResizeInput message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: zed.scene.IResizeInput, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ResizeInput message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ResizeInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): zed.scene.ResizeInput;

            /**
             * Decodes a ResizeInput message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ResizeInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): zed.scene.ResizeInput;

            /**
             * Verifies a ResizeInput message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ResizeInput message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ResizeInput
             */
            public static fromObject(object: { [k: string]: any }): zed.scene.ResizeInput;

            /**
             * Creates a plain object from a ResizeInput message. Also converts values to other types if specified.
             * @param message ResizeInput
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: zed.scene.ResizeInput, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ResizeInput to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };

            /**
             * Gets the default type url for ResizeInput
             * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns The default type url
             */
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }
    }
}
