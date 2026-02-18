/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
import * as $protobuf from "protobufjs/minimal";

// Common aliases
const $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
const $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

export const zed = $root.zed = (() => {

    /**
     * Namespace zed.
     * @exports zed
     * @namespace
     */
    const zed = {};

    zed.scene = (function() {

        /**
         * Namespace scene.
         * @memberof zed
         * @namespace
         */
        const scene = {};

        scene.FrameMessage = (function() {

            /**
             * Properties of a FrameMessage.
             * @memberof zed.scene
             * @interface IFrameMessage
             * @property {number|Long|null} [frameId] FrameMessage frameId
             * @property {number|null} [viewportWidth] FrameMessage viewportWidth
             * @property {number|null} [viewportHeight] FrameMessage viewportHeight
             * @property {number|null} [scaleFactor] FrameMessage scaleFactor
             * @property {Array.<zed.scene.IAtlasEntry>|null} [atlasEntries] FrameMessage atlasEntries
             * @property {zed.scene.ISceneBody|null} [scene] FrameMessage scene
             * @property {zed.scene.IHsla|null} [backgroundColor] FrameMessage backgroundColor
             * @property {zed.scene.IThemeHints|null} [themeHints] FrameMessage themeHints
             */

            /**
             * Constructs a new FrameMessage.
             * @memberof zed.scene
             * @classdesc Represents a FrameMessage.
             * @implements IFrameMessage
             * @constructor
             * @param {zed.scene.IFrameMessage=} [properties] Properties to set
             */
            function FrameMessage(properties) {
                this.atlasEntries = [];
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * FrameMessage frameId.
             * @member {number|Long} frameId
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.frameId = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

            /**
             * FrameMessage viewportWidth.
             * @member {number} viewportWidth
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.viewportWidth = 0;

            /**
             * FrameMessage viewportHeight.
             * @member {number} viewportHeight
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.viewportHeight = 0;

            /**
             * FrameMessage scaleFactor.
             * @member {number} scaleFactor
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.scaleFactor = 0;

            /**
             * FrameMessage atlasEntries.
             * @member {Array.<zed.scene.IAtlasEntry>} atlasEntries
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.atlasEntries = $util.emptyArray;

            /**
             * FrameMessage scene.
             * @member {zed.scene.ISceneBody|null|undefined} scene
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.scene = null;

            /**
             * FrameMessage backgroundColor.
             * @member {zed.scene.IHsla|null|undefined} backgroundColor
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.backgroundColor = null;

            /**
             * FrameMessage themeHints.
             * @member {zed.scene.IThemeHints|null|undefined} themeHints
             * @memberof zed.scene.FrameMessage
             * @instance
             */
            FrameMessage.prototype.themeHints = null;

            /**
             * Creates a new FrameMessage instance using the specified properties.
             * @function create
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {zed.scene.IFrameMessage=} [properties] Properties to set
             * @returns {zed.scene.FrameMessage} FrameMessage instance
             */
            FrameMessage.create = function create(properties) {
                return new FrameMessage(properties);
            };

            /**
             * Encodes the specified FrameMessage message. Does not implicitly {@link zed.scene.FrameMessage.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {zed.scene.IFrameMessage} message FrameMessage message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            FrameMessage.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.frameId != null && Object.hasOwnProperty.call(message, "frameId"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint64(message.frameId);
                if (message.viewportWidth != null && Object.hasOwnProperty.call(message, "viewportWidth"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.viewportWidth);
                if (message.viewportHeight != null && Object.hasOwnProperty.call(message, "viewportHeight"))
                    writer.uint32(/* id 3, wireType 5 =*/29).float(message.viewportHeight);
                if (message.scaleFactor != null && Object.hasOwnProperty.call(message, "scaleFactor"))
                    writer.uint32(/* id 4, wireType 5 =*/37).float(message.scaleFactor);
                if (message.atlasEntries != null && message.atlasEntries.length)
                    for (let i = 0; i < message.atlasEntries.length; ++i)
                        $root.zed.scene.AtlasEntry.encode(message.atlasEntries[i], writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.scene != null && Object.hasOwnProperty.call(message, "scene"))
                    $root.zed.scene.SceneBody.encode(message.scene, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                if (message.backgroundColor != null && Object.hasOwnProperty.call(message, "backgroundColor"))
                    $root.zed.scene.Hsla.encode(message.backgroundColor, writer.uint32(/* id 7, wireType 2 =*/58).fork()).ldelim();
                if (message.themeHints != null && Object.hasOwnProperty.call(message, "themeHints"))
                    $root.zed.scene.ThemeHints.encode(message.themeHints, writer.uint32(/* id 8, wireType 2 =*/66).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified FrameMessage message, length delimited. Does not implicitly {@link zed.scene.FrameMessage.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {zed.scene.IFrameMessage} message FrameMessage message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            FrameMessage.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a FrameMessage message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.FrameMessage} FrameMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            FrameMessage.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.FrameMessage();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.frameId = reader.uint64();
                            break;
                        }
                    case 2: {
                            message.viewportWidth = reader.float();
                            break;
                        }
                    case 3: {
                            message.viewportHeight = reader.float();
                            break;
                        }
                    case 4: {
                            message.scaleFactor = reader.float();
                            break;
                        }
                    case 5: {
                            if (!(message.atlasEntries && message.atlasEntries.length))
                                message.atlasEntries = [];
                            message.atlasEntries.push($root.zed.scene.AtlasEntry.decode(reader, reader.uint32()));
                            break;
                        }
                    case 6: {
                            message.scene = $root.zed.scene.SceneBody.decode(reader, reader.uint32());
                            break;
                        }
                    case 7: {
                            message.backgroundColor = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 8: {
                            message.themeHints = $root.zed.scene.ThemeHints.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a FrameMessage message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.FrameMessage} FrameMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            FrameMessage.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a FrameMessage message.
             * @function verify
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            FrameMessage.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.frameId != null && message.hasOwnProperty("frameId"))
                    if (!$util.isInteger(message.frameId) && !(message.frameId && $util.isInteger(message.frameId.low) && $util.isInteger(message.frameId.high)))
                        return "frameId: integer|Long expected";
                if (message.viewportWidth != null && message.hasOwnProperty("viewportWidth"))
                    if (typeof message.viewportWidth !== "number")
                        return "viewportWidth: number expected";
                if (message.viewportHeight != null && message.hasOwnProperty("viewportHeight"))
                    if (typeof message.viewportHeight !== "number")
                        return "viewportHeight: number expected";
                if (message.scaleFactor != null && message.hasOwnProperty("scaleFactor"))
                    if (typeof message.scaleFactor !== "number")
                        return "scaleFactor: number expected";
                if (message.atlasEntries != null && message.hasOwnProperty("atlasEntries")) {
                    if (!Array.isArray(message.atlasEntries))
                        return "atlasEntries: array expected";
                    for (let i = 0; i < message.atlasEntries.length; ++i) {
                        let error = $root.zed.scene.AtlasEntry.verify(message.atlasEntries[i]);
                        if (error)
                            return "atlasEntries." + error;
                    }
                }
                if (message.scene != null && message.hasOwnProperty("scene")) {
                    let error = $root.zed.scene.SceneBody.verify(message.scene);
                    if (error)
                        return "scene." + error;
                }
                if (message.backgroundColor != null && message.hasOwnProperty("backgroundColor")) {
                    let error = $root.zed.scene.Hsla.verify(message.backgroundColor);
                    if (error)
                        return "backgroundColor." + error;
                }
                if (message.themeHints != null && message.hasOwnProperty("themeHints")) {
                    let error = $root.zed.scene.ThemeHints.verify(message.themeHints);
                    if (error)
                        return "themeHints." + error;
                }
                return null;
            };

            /**
             * Creates a FrameMessage message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.FrameMessage} FrameMessage
             */
            FrameMessage.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.FrameMessage)
                    return object;
                let message = new $root.zed.scene.FrameMessage();
                if (object.frameId != null)
                    if ($util.Long)
                        (message.frameId = $util.Long.fromValue(object.frameId)).unsigned = true;
                    else if (typeof object.frameId === "string")
                        message.frameId = parseInt(object.frameId, 10);
                    else if (typeof object.frameId === "number")
                        message.frameId = object.frameId;
                    else if (typeof object.frameId === "object")
                        message.frameId = new $util.LongBits(object.frameId.low >>> 0, object.frameId.high >>> 0).toNumber(true);
                if (object.viewportWidth != null)
                    message.viewportWidth = Number(object.viewportWidth);
                if (object.viewportHeight != null)
                    message.viewportHeight = Number(object.viewportHeight);
                if (object.scaleFactor != null)
                    message.scaleFactor = Number(object.scaleFactor);
                if (object.atlasEntries) {
                    if (!Array.isArray(object.atlasEntries))
                        throw TypeError(".zed.scene.FrameMessage.atlasEntries: array expected");
                    message.atlasEntries = [];
                    for (let i = 0; i < object.atlasEntries.length; ++i) {
                        if (typeof object.atlasEntries[i] !== "object")
                            throw TypeError(".zed.scene.FrameMessage.atlasEntries: object expected");
                        message.atlasEntries[i] = $root.zed.scene.AtlasEntry.fromObject(object.atlasEntries[i]);
                    }
                }
                if (object.scene != null) {
                    if (typeof object.scene !== "object")
                        throw TypeError(".zed.scene.FrameMessage.scene: object expected");
                    message.scene = $root.zed.scene.SceneBody.fromObject(object.scene);
                }
                if (object.backgroundColor != null) {
                    if (typeof object.backgroundColor !== "object")
                        throw TypeError(".zed.scene.FrameMessage.backgroundColor: object expected");
                    message.backgroundColor = $root.zed.scene.Hsla.fromObject(object.backgroundColor);
                }
                if (object.themeHints != null) {
                    if (typeof object.themeHints !== "object")
                        throw TypeError(".zed.scene.FrameMessage.themeHints: object expected");
                    message.themeHints = $root.zed.scene.ThemeHints.fromObject(object.themeHints);
                }
                return message;
            };

            /**
             * Creates a plain object from a FrameMessage message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {zed.scene.FrameMessage} message FrameMessage
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            FrameMessage.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.arrays || options.defaults)
                    object.atlasEntries = [];
                if (options.defaults) {
                    if ($util.Long) {
                        let long = new $util.Long(0, 0, true);
                        object.frameId = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                    } else
                        object.frameId = options.longs === String ? "0" : 0;
                    object.viewportWidth = 0;
                    object.viewportHeight = 0;
                    object.scaleFactor = 0;
                    object.scene = null;
                    object.backgroundColor = null;
                    object.themeHints = null;
                }
                if (message.frameId != null && message.hasOwnProperty("frameId"))
                    if (typeof message.frameId === "number")
                        object.frameId = options.longs === String ? String(message.frameId) : message.frameId;
                    else
                        object.frameId = options.longs === String ? $util.Long.prototype.toString.call(message.frameId) : options.longs === Number ? new $util.LongBits(message.frameId.low >>> 0, message.frameId.high >>> 0).toNumber(true) : message.frameId;
                if (message.viewportWidth != null && message.hasOwnProperty("viewportWidth"))
                    object.viewportWidth = options.json && !isFinite(message.viewportWidth) ? String(message.viewportWidth) : message.viewportWidth;
                if (message.viewportHeight != null && message.hasOwnProperty("viewportHeight"))
                    object.viewportHeight = options.json && !isFinite(message.viewportHeight) ? String(message.viewportHeight) : message.viewportHeight;
                if (message.scaleFactor != null && message.hasOwnProperty("scaleFactor"))
                    object.scaleFactor = options.json && !isFinite(message.scaleFactor) ? String(message.scaleFactor) : message.scaleFactor;
                if (message.atlasEntries && message.atlasEntries.length) {
                    object.atlasEntries = [];
                    for (let j = 0; j < message.atlasEntries.length; ++j)
                        object.atlasEntries[j] = $root.zed.scene.AtlasEntry.toObject(message.atlasEntries[j], options);
                }
                if (message.scene != null && message.hasOwnProperty("scene"))
                    object.scene = $root.zed.scene.SceneBody.toObject(message.scene, options);
                if (message.backgroundColor != null && message.hasOwnProperty("backgroundColor"))
                    object.backgroundColor = $root.zed.scene.Hsla.toObject(message.backgroundColor, options);
                if (message.themeHints != null && message.hasOwnProperty("themeHints"))
                    object.themeHints = $root.zed.scene.ThemeHints.toObject(message.themeHints, options);
                return object;
            };

            /**
             * Converts this FrameMessage to JSON.
             * @function toJSON
             * @memberof zed.scene.FrameMessage
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            FrameMessage.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for FrameMessage
             * @function getTypeUrl
             * @memberof zed.scene.FrameMessage
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            FrameMessage.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.FrameMessage";
            };

            return FrameMessage;
        })();

        scene.ThemeHints = (function() {

            /**
             * Properties of a ThemeHints.
             * @memberof zed.scene
             * @interface IThemeHints
             * @property {string|null} [appearance] ThemeHints appearance
             * @property {number|null} [backgroundRgb] ThemeHints backgroundRgb
             * @property {string|null} [backgroundCss] ThemeHints backgroundCss
             * @property {string|null} [backgroundAppearance] ThemeHints backgroundAppearance
             */

            /**
             * Constructs a new ThemeHints.
             * @memberof zed.scene
             * @classdesc Represents a ThemeHints.
             * @implements IThemeHints
             * @constructor
             * @param {zed.scene.IThemeHints=} [properties] Properties to set
             */
            function ThemeHints(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ThemeHints appearance.
             * @member {string} appearance
             * @memberof zed.scene.ThemeHints
             * @instance
             */
            ThemeHints.prototype.appearance = "";

            /**
             * ThemeHints backgroundRgb.
             * @member {number} backgroundRgb
             * @memberof zed.scene.ThemeHints
             * @instance
             */
            ThemeHints.prototype.backgroundRgb = 0;

            /**
             * ThemeHints backgroundCss.
             * @member {string} backgroundCss
             * @memberof zed.scene.ThemeHints
             * @instance
             */
            ThemeHints.prototype.backgroundCss = "";

            /**
             * ThemeHints backgroundAppearance.
             * @member {string} backgroundAppearance
             * @memberof zed.scene.ThemeHints
             * @instance
             */
            ThemeHints.prototype.backgroundAppearance = "";

            /**
             * Creates a new ThemeHints instance using the specified properties.
             * @function create
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {zed.scene.IThemeHints=} [properties] Properties to set
             * @returns {zed.scene.ThemeHints} ThemeHints instance
             */
            ThemeHints.create = function create(properties) {
                return new ThemeHints(properties);
            };

            /**
             * Encodes the specified ThemeHints message. Does not implicitly {@link zed.scene.ThemeHints.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {zed.scene.IThemeHints} message ThemeHints message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ThemeHints.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.appearance != null && Object.hasOwnProperty.call(message, "appearance"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.appearance);
                if (message.backgroundRgb != null && Object.hasOwnProperty.call(message, "backgroundRgb"))
                    writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.backgroundRgb);
                if (message.backgroundCss != null && Object.hasOwnProperty.call(message, "backgroundCss"))
                    writer.uint32(/* id 3, wireType 2 =*/26).string(message.backgroundCss);
                if (message.backgroundAppearance != null && Object.hasOwnProperty.call(message, "backgroundAppearance"))
                    writer.uint32(/* id 4, wireType 2 =*/34).string(message.backgroundAppearance);
                return writer;
            };

            /**
             * Encodes the specified ThemeHints message, length delimited. Does not implicitly {@link zed.scene.ThemeHints.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {zed.scene.IThemeHints} message ThemeHints message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ThemeHints.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a ThemeHints message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.ThemeHints} ThemeHints
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ThemeHints.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.ThemeHints();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.appearance = reader.string();
                            break;
                        }
                    case 2: {
                            message.backgroundRgb = reader.uint32();
                            break;
                        }
                    case 3: {
                            message.backgroundCss = reader.string();
                            break;
                        }
                    case 4: {
                            message.backgroundAppearance = reader.string();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a ThemeHints message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.ThemeHints} ThemeHints
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ThemeHints.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a ThemeHints message.
             * @function verify
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ThemeHints.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.appearance != null && message.hasOwnProperty("appearance"))
                    if (!$util.isString(message.appearance))
                        return "appearance: string expected";
                if (message.backgroundRgb != null && message.hasOwnProperty("backgroundRgb"))
                    if (!$util.isInteger(message.backgroundRgb))
                        return "backgroundRgb: integer expected";
                if (message.backgroundCss != null && message.hasOwnProperty("backgroundCss"))
                    if (!$util.isString(message.backgroundCss))
                        return "backgroundCss: string expected";
                if (message.backgroundAppearance != null && message.hasOwnProperty("backgroundAppearance"))
                    if (!$util.isString(message.backgroundAppearance))
                        return "backgroundAppearance: string expected";
                return null;
            };

            /**
             * Creates a ThemeHints message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.ThemeHints} ThemeHints
             */
            ThemeHints.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.ThemeHints)
                    return object;
                let message = new $root.zed.scene.ThemeHints();
                if (object.appearance != null)
                    message.appearance = String(object.appearance);
                if (object.backgroundRgb != null)
                    message.backgroundRgb = object.backgroundRgb >>> 0;
                if (object.backgroundCss != null)
                    message.backgroundCss = String(object.backgroundCss);
                if (object.backgroundAppearance != null)
                    message.backgroundAppearance = String(object.backgroundAppearance);
                return message;
            };

            /**
             * Creates a plain object from a ThemeHints message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {zed.scene.ThemeHints} message ThemeHints
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ThemeHints.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.appearance = "";
                    object.backgroundRgb = 0;
                    object.backgroundCss = "";
                    object.backgroundAppearance = "";
                }
                if (message.appearance != null && message.hasOwnProperty("appearance"))
                    object.appearance = message.appearance;
                if (message.backgroundRgb != null && message.hasOwnProperty("backgroundRgb"))
                    object.backgroundRgb = message.backgroundRgb;
                if (message.backgroundCss != null && message.hasOwnProperty("backgroundCss"))
                    object.backgroundCss = message.backgroundCss;
                if (message.backgroundAppearance != null && message.hasOwnProperty("backgroundAppearance"))
                    object.backgroundAppearance = message.backgroundAppearance;
                return object;
            };

            /**
             * Converts this ThemeHints to JSON.
             * @function toJSON
             * @memberof zed.scene.ThemeHints
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ThemeHints.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for ThemeHints
             * @function getTypeUrl
             * @memberof zed.scene.ThemeHints
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            ThemeHints.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.ThemeHints";
            };

            return ThemeHints;
        })();

        scene.SceneBody = (function() {

            /**
             * Properties of a SceneBody.
             * @memberof zed.scene
             * @interface ISceneBody
             * @property {Array.<zed.scene.IShadow>|null} [shadows] SceneBody shadows
             * @property {Array.<zed.scene.IQuad>|null} [quads] SceneBody quads
             * @property {Array.<zed.scene.IUnderline>|null} [underlines] SceneBody underlines
             * @property {Array.<zed.scene.IMonochromeSprite>|null} [monochromeSprites] SceneBody monochromeSprites
             * @property {Array.<zed.scene.ISubpixelSprite>|null} [subpixelSprites] SceneBody subpixelSprites
             * @property {Array.<zed.scene.IPolychromeSprite>|null} [polychromeSprites] SceneBody polychromeSprites
             * @property {Array.<zed.scene.IPath>|null} [paths] SceneBody paths
             */

            /**
             * Constructs a new SceneBody.
             * @memberof zed.scene
             * @classdesc Represents a SceneBody.
             * @implements ISceneBody
             * @constructor
             * @param {zed.scene.ISceneBody=} [properties] Properties to set
             */
            function SceneBody(properties) {
                this.shadows = [];
                this.quads = [];
                this.underlines = [];
                this.monochromeSprites = [];
                this.subpixelSprites = [];
                this.polychromeSprites = [];
                this.paths = [];
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * SceneBody shadows.
             * @member {Array.<zed.scene.IShadow>} shadows
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.shadows = $util.emptyArray;

            /**
             * SceneBody quads.
             * @member {Array.<zed.scene.IQuad>} quads
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.quads = $util.emptyArray;

            /**
             * SceneBody underlines.
             * @member {Array.<zed.scene.IUnderline>} underlines
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.underlines = $util.emptyArray;

            /**
             * SceneBody monochromeSprites.
             * @member {Array.<zed.scene.IMonochromeSprite>} monochromeSprites
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.monochromeSprites = $util.emptyArray;

            /**
             * SceneBody subpixelSprites.
             * @member {Array.<zed.scene.ISubpixelSprite>} subpixelSprites
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.subpixelSprites = $util.emptyArray;

            /**
             * SceneBody polychromeSprites.
             * @member {Array.<zed.scene.IPolychromeSprite>} polychromeSprites
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.polychromeSprites = $util.emptyArray;

            /**
             * SceneBody paths.
             * @member {Array.<zed.scene.IPath>} paths
             * @memberof zed.scene.SceneBody
             * @instance
             */
            SceneBody.prototype.paths = $util.emptyArray;

            /**
             * Creates a new SceneBody instance using the specified properties.
             * @function create
             * @memberof zed.scene.SceneBody
             * @static
             * @param {zed.scene.ISceneBody=} [properties] Properties to set
             * @returns {zed.scene.SceneBody} SceneBody instance
             */
            SceneBody.create = function create(properties) {
                return new SceneBody(properties);
            };

            /**
             * Encodes the specified SceneBody message. Does not implicitly {@link zed.scene.SceneBody.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.SceneBody
             * @static
             * @param {zed.scene.ISceneBody} message SceneBody message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SceneBody.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.shadows != null && message.shadows.length)
                    for (let i = 0; i < message.shadows.length; ++i)
                        $root.zed.scene.Shadow.encode(message.shadows[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.quads != null && message.quads.length)
                    for (let i = 0; i < message.quads.length; ++i)
                        $root.zed.scene.Quad.encode(message.quads[i], writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.underlines != null && message.underlines.length)
                    for (let i = 0; i < message.underlines.length; ++i)
                        $root.zed.scene.Underline.encode(message.underlines[i], writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.monochromeSprites != null && message.monochromeSprites.length)
                    for (let i = 0; i < message.monochromeSprites.length; ++i)
                        $root.zed.scene.MonochromeSprite.encode(message.monochromeSprites[i], writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.subpixelSprites != null && message.subpixelSprites.length)
                    for (let i = 0; i < message.subpixelSprites.length; ++i)
                        $root.zed.scene.SubpixelSprite.encode(message.subpixelSprites[i], writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.polychromeSprites != null && message.polychromeSprites.length)
                    for (let i = 0; i < message.polychromeSprites.length; ++i)
                        $root.zed.scene.PolychromeSprite.encode(message.polychromeSprites[i], writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                if (message.paths != null && message.paths.length)
                    for (let i = 0; i < message.paths.length; ++i)
                        $root.zed.scene.Path.encode(message.paths[i], writer.uint32(/* id 7, wireType 2 =*/58).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified SceneBody message, length delimited. Does not implicitly {@link zed.scene.SceneBody.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.SceneBody
             * @static
             * @param {zed.scene.ISceneBody} message SceneBody message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SceneBody.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a SceneBody message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.SceneBody
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.SceneBody} SceneBody
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SceneBody.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.SceneBody();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            if (!(message.shadows && message.shadows.length))
                                message.shadows = [];
                            message.shadows.push($root.zed.scene.Shadow.decode(reader, reader.uint32()));
                            break;
                        }
                    case 2: {
                            if (!(message.quads && message.quads.length))
                                message.quads = [];
                            message.quads.push($root.zed.scene.Quad.decode(reader, reader.uint32()));
                            break;
                        }
                    case 3: {
                            if (!(message.underlines && message.underlines.length))
                                message.underlines = [];
                            message.underlines.push($root.zed.scene.Underline.decode(reader, reader.uint32()));
                            break;
                        }
                    case 4: {
                            if (!(message.monochromeSprites && message.monochromeSprites.length))
                                message.monochromeSprites = [];
                            message.monochromeSprites.push($root.zed.scene.MonochromeSprite.decode(reader, reader.uint32()));
                            break;
                        }
                    case 5: {
                            if (!(message.subpixelSprites && message.subpixelSprites.length))
                                message.subpixelSprites = [];
                            message.subpixelSprites.push($root.zed.scene.SubpixelSprite.decode(reader, reader.uint32()));
                            break;
                        }
                    case 6: {
                            if (!(message.polychromeSprites && message.polychromeSprites.length))
                                message.polychromeSprites = [];
                            message.polychromeSprites.push($root.zed.scene.PolychromeSprite.decode(reader, reader.uint32()));
                            break;
                        }
                    case 7: {
                            if (!(message.paths && message.paths.length))
                                message.paths = [];
                            message.paths.push($root.zed.scene.Path.decode(reader, reader.uint32()));
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a SceneBody message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.SceneBody
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.SceneBody} SceneBody
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SceneBody.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a SceneBody message.
             * @function verify
             * @memberof zed.scene.SceneBody
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            SceneBody.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.shadows != null && message.hasOwnProperty("shadows")) {
                    if (!Array.isArray(message.shadows))
                        return "shadows: array expected";
                    for (let i = 0; i < message.shadows.length; ++i) {
                        let error = $root.zed.scene.Shadow.verify(message.shadows[i]);
                        if (error)
                            return "shadows." + error;
                    }
                }
                if (message.quads != null && message.hasOwnProperty("quads")) {
                    if (!Array.isArray(message.quads))
                        return "quads: array expected";
                    for (let i = 0; i < message.quads.length; ++i) {
                        let error = $root.zed.scene.Quad.verify(message.quads[i]);
                        if (error)
                            return "quads." + error;
                    }
                }
                if (message.underlines != null && message.hasOwnProperty("underlines")) {
                    if (!Array.isArray(message.underlines))
                        return "underlines: array expected";
                    for (let i = 0; i < message.underlines.length; ++i) {
                        let error = $root.zed.scene.Underline.verify(message.underlines[i]);
                        if (error)
                            return "underlines." + error;
                    }
                }
                if (message.monochromeSprites != null && message.hasOwnProperty("monochromeSprites")) {
                    if (!Array.isArray(message.monochromeSprites))
                        return "monochromeSprites: array expected";
                    for (let i = 0; i < message.monochromeSprites.length; ++i) {
                        let error = $root.zed.scene.MonochromeSprite.verify(message.monochromeSprites[i]);
                        if (error)
                            return "monochromeSprites." + error;
                    }
                }
                if (message.subpixelSprites != null && message.hasOwnProperty("subpixelSprites")) {
                    if (!Array.isArray(message.subpixelSprites))
                        return "subpixelSprites: array expected";
                    for (let i = 0; i < message.subpixelSprites.length; ++i) {
                        let error = $root.zed.scene.SubpixelSprite.verify(message.subpixelSprites[i]);
                        if (error)
                            return "subpixelSprites." + error;
                    }
                }
                if (message.polychromeSprites != null && message.hasOwnProperty("polychromeSprites")) {
                    if (!Array.isArray(message.polychromeSprites))
                        return "polychromeSprites: array expected";
                    for (let i = 0; i < message.polychromeSprites.length; ++i) {
                        let error = $root.zed.scene.PolychromeSprite.verify(message.polychromeSprites[i]);
                        if (error)
                            return "polychromeSprites." + error;
                    }
                }
                if (message.paths != null && message.hasOwnProperty("paths")) {
                    if (!Array.isArray(message.paths))
                        return "paths: array expected";
                    for (let i = 0; i < message.paths.length; ++i) {
                        let error = $root.zed.scene.Path.verify(message.paths[i]);
                        if (error)
                            return "paths." + error;
                    }
                }
                return null;
            };

            /**
             * Creates a SceneBody message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.SceneBody
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.SceneBody} SceneBody
             */
            SceneBody.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.SceneBody)
                    return object;
                let message = new $root.zed.scene.SceneBody();
                if (object.shadows) {
                    if (!Array.isArray(object.shadows))
                        throw TypeError(".zed.scene.SceneBody.shadows: array expected");
                    message.shadows = [];
                    for (let i = 0; i < object.shadows.length; ++i) {
                        if (typeof object.shadows[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.shadows: object expected");
                        message.shadows[i] = $root.zed.scene.Shadow.fromObject(object.shadows[i]);
                    }
                }
                if (object.quads) {
                    if (!Array.isArray(object.quads))
                        throw TypeError(".zed.scene.SceneBody.quads: array expected");
                    message.quads = [];
                    for (let i = 0; i < object.quads.length; ++i) {
                        if (typeof object.quads[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.quads: object expected");
                        message.quads[i] = $root.zed.scene.Quad.fromObject(object.quads[i]);
                    }
                }
                if (object.underlines) {
                    if (!Array.isArray(object.underlines))
                        throw TypeError(".zed.scene.SceneBody.underlines: array expected");
                    message.underlines = [];
                    for (let i = 0; i < object.underlines.length; ++i) {
                        if (typeof object.underlines[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.underlines: object expected");
                        message.underlines[i] = $root.zed.scene.Underline.fromObject(object.underlines[i]);
                    }
                }
                if (object.monochromeSprites) {
                    if (!Array.isArray(object.monochromeSprites))
                        throw TypeError(".zed.scene.SceneBody.monochromeSprites: array expected");
                    message.monochromeSprites = [];
                    for (let i = 0; i < object.monochromeSprites.length; ++i) {
                        if (typeof object.monochromeSprites[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.monochromeSprites: object expected");
                        message.monochromeSprites[i] = $root.zed.scene.MonochromeSprite.fromObject(object.monochromeSprites[i]);
                    }
                }
                if (object.subpixelSprites) {
                    if (!Array.isArray(object.subpixelSprites))
                        throw TypeError(".zed.scene.SceneBody.subpixelSprites: array expected");
                    message.subpixelSprites = [];
                    for (let i = 0; i < object.subpixelSprites.length; ++i) {
                        if (typeof object.subpixelSprites[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.subpixelSprites: object expected");
                        message.subpixelSprites[i] = $root.zed.scene.SubpixelSprite.fromObject(object.subpixelSprites[i]);
                    }
                }
                if (object.polychromeSprites) {
                    if (!Array.isArray(object.polychromeSprites))
                        throw TypeError(".zed.scene.SceneBody.polychromeSprites: array expected");
                    message.polychromeSprites = [];
                    for (let i = 0; i < object.polychromeSprites.length; ++i) {
                        if (typeof object.polychromeSprites[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.polychromeSprites: object expected");
                        message.polychromeSprites[i] = $root.zed.scene.PolychromeSprite.fromObject(object.polychromeSprites[i]);
                    }
                }
                if (object.paths) {
                    if (!Array.isArray(object.paths))
                        throw TypeError(".zed.scene.SceneBody.paths: array expected");
                    message.paths = [];
                    for (let i = 0; i < object.paths.length; ++i) {
                        if (typeof object.paths[i] !== "object")
                            throw TypeError(".zed.scene.SceneBody.paths: object expected");
                        message.paths[i] = $root.zed.scene.Path.fromObject(object.paths[i]);
                    }
                }
                return message;
            };

            /**
             * Creates a plain object from a SceneBody message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.SceneBody
             * @static
             * @param {zed.scene.SceneBody} message SceneBody
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            SceneBody.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.arrays || options.defaults) {
                    object.shadows = [];
                    object.quads = [];
                    object.underlines = [];
                    object.monochromeSprites = [];
                    object.subpixelSprites = [];
                    object.polychromeSprites = [];
                    object.paths = [];
                }
                if (message.shadows && message.shadows.length) {
                    object.shadows = [];
                    for (let j = 0; j < message.shadows.length; ++j)
                        object.shadows[j] = $root.zed.scene.Shadow.toObject(message.shadows[j], options);
                }
                if (message.quads && message.quads.length) {
                    object.quads = [];
                    for (let j = 0; j < message.quads.length; ++j)
                        object.quads[j] = $root.zed.scene.Quad.toObject(message.quads[j], options);
                }
                if (message.underlines && message.underlines.length) {
                    object.underlines = [];
                    for (let j = 0; j < message.underlines.length; ++j)
                        object.underlines[j] = $root.zed.scene.Underline.toObject(message.underlines[j], options);
                }
                if (message.monochromeSprites && message.monochromeSprites.length) {
                    object.monochromeSprites = [];
                    for (let j = 0; j < message.monochromeSprites.length; ++j)
                        object.monochromeSprites[j] = $root.zed.scene.MonochromeSprite.toObject(message.monochromeSprites[j], options);
                }
                if (message.subpixelSprites && message.subpixelSprites.length) {
                    object.subpixelSprites = [];
                    for (let j = 0; j < message.subpixelSprites.length; ++j)
                        object.subpixelSprites[j] = $root.zed.scene.SubpixelSprite.toObject(message.subpixelSprites[j], options);
                }
                if (message.polychromeSprites && message.polychromeSprites.length) {
                    object.polychromeSprites = [];
                    for (let j = 0; j < message.polychromeSprites.length; ++j)
                        object.polychromeSprites[j] = $root.zed.scene.PolychromeSprite.toObject(message.polychromeSprites[j], options);
                }
                if (message.paths && message.paths.length) {
                    object.paths = [];
                    for (let j = 0; j < message.paths.length; ++j)
                        object.paths[j] = $root.zed.scene.Path.toObject(message.paths[j], options);
                }
                return object;
            };

            /**
             * Converts this SceneBody to JSON.
             * @function toJSON
             * @memberof zed.scene.SceneBody
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            SceneBody.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for SceneBody
             * @function getTypeUrl
             * @memberof zed.scene.SceneBody
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            SceneBody.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.SceneBody";
            };

            return SceneBody;
        })();

        scene.AtlasEntry = (function() {

            /**
             * Properties of an AtlasEntry.
             * @memberof zed.scene
             * @interface IAtlasEntry
             * @property {zed.scene.IAtlasTextureId|null} [textureId] AtlasEntry textureId
             * @property {zed.scene.IAtlasBounds|null} [bounds] AtlasEntry bounds
             * @property {number|null} [format] AtlasEntry format
             * @property {Uint8Array|null} [pixelData] AtlasEntry pixelData
             */

            /**
             * Constructs a new AtlasEntry.
             * @memberof zed.scene
             * @classdesc Represents an AtlasEntry.
             * @implements IAtlasEntry
             * @constructor
             * @param {zed.scene.IAtlasEntry=} [properties] Properties to set
             */
            function AtlasEntry(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * AtlasEntry textureId.
             * @member {zed.scene.IAtlasTextureId|null|undefined} textureId
             * @memberof zed.scene.AtlasEntry
             * @instance
             */
            AtlasEntry.prototype.textureId = null;

            /**
             * AtlasEntry bounds.
             * @member {zed.scene.IAtlasBounds|null|undefined} bounds
             * @memberof zed.scene.AtlasEntry
             * @instance
             */
            AtlasEntry.prototype.bounds = null;

            /**
             * AtlasEntry format.
             * @member {number} format
             * @memberof zed.scene.AtlasEntry
             * @instance
             */
            AtlasEntry.prototype.format = 0;

            /**
             * AtlasEntry pixelData.
             * @member {Uint8Array} pixelData
             * @memberof zed.scene.AtlasEntry
             * @instance
             */
            AtlasEntry.prototype.pixelData = $util.newBuffer([]);

            /**
             * Creates a new AtlasEntry instance using the specified properties.
             * @function create
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {zed.scene.IAtlasEntry=} [properties] Properties to set
             * @returns {zed.scene.AtlasEntry} AtlasEntry instance
             */
            AtlasEntry.create = function create(properties) {
                return new AtlasEntry(properties);
            };

            /**
             * Encodes the specified AtlasEntry message. Does not implicitly {@link zed.scene.AtlasEntry.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {zed.scene.IAtlasEntry} message AtlasEntry message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasEntry.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.textureId != null && Object.hasOwnProperty.call(message, "textureId"))
                    $root.zed.scene.AtlasTextureId.encode(message.textureId, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.AtlasBounds.encode(message.bounds, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.format != null && Object.hasOwnProperty.call(message, "format"))
                    writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.format);
                if (message.pixelData != null && Object.hasOwnProperty.call(message, "pixelData"))
                    writer.uint32(/* id 4, wireType 2 =*/34).bytes(message.pixelData);
                return writer;
            };

            /**
             * Encodes the specified AtlasEntry message, length delimited. Does not implicitly {@link zed.scene.AtlasEntry.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {zed.scene.IAtlasEntry} message AtlasEntry message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasEntry.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an AtlasEntry message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.AtlasEntry} AtlasEntry
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasEntry.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.AtlasEntry();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.textureId = $root.zed.scene.AtlasTextureId.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.bounds = $root.zed.scene.AtlasBounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.format = reader.uint32();
                            break;
                        }
                    case 4: {
                            message.pixelData = reader.bytes();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an AtlasEntry message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.AtlasEntry} AtlasEntry
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasEntry.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an AtlasEntry message.
             * @function verify
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            AtlasEntry.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.textureId != null && message.hasOwnProperty("textureId")) {
                    let error = $root.zed.scene.AtlasTextureId.verify(message.textureId);
                    if (error)
                        return "textureId." + error;
                }
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.AtlasBounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.format != null && message.hasOwnProperty("format"))
                    if (!$util.isInteger(message.format))
                        return "format: integer expected";
                if (message.pixelData != null && message.hasOwnProperty("pixelData"))
                    if (!(message.pixelData && typeof message.pixelData.length === "number" || $util.isString(message.pixelData)))
                        return "pixelData: buffer expected";
                return null;
            };

            /**
             * Creates an AtlasEntry message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.AtlasEntry} AtlasEntry
             */
            AtlasEntry.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.AtlasEntry)
                    return object;
                let message = new $root.zed.scene.AtlasEntry();
                if (object.textureId != null) {
                    if (typeof object.textureId !== "object")
                        throw TypeError(".zed.scene.AtlasEntry.textureId: object expected");
                    message.textureId = $root.zed.scene.AtlasTextureId.fromObject(object.textureId);
                }
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.AtlasEntry.bounds: object expected");
                    message.bounds = $root.zed.scene.AtlasBounds.fromObject(object.bounds);
                }
                if (object.format != null)
                    message.format = object.format >>> 0;
                if (object.pixelData != null)
                    if (typeof object.pixelData === "string")
                        $util.base64.decode(object.pixelData, message.pixelData = $util.newBuffer($util.base64.length(object.pixelData)), 0);
                    else if (object.pixelData.length >= 0)
                        message.pixelData = object.pixelData;
                return message;
            };

            /**
             * Creates a plain object from an AtlasEntry message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {zed.scene.AtlasEntry} message AtlasEntry
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            AtlasEntry.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.textureId = null;
                    object.bounds = null;
                    object.format = 0;
                    if (options.bytes === String)
                        object.pixelData = "";
                    else {
                        object.pixelData = [];
                        if (options.bytes !== Array)
                            object.pixelData = $util.newBuffer(object.pixelData);
                    }
                }
                if (message.textureId != null && message.hasOwnProperty("textureId"))
                    object.textureId = $root.zed.scene.AtlasTextureId.toObject(message.textureId, options);
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.AtlasBounds.toObject(message.bounds, options);
                if (message.format != null && message.hasOwnProperty("format"))
                    object.format = message.format;
                if (message.pixelData != null && message.hasOwnProperty("pixelData"))
                    object.pixelData = options.bytes === String ? $util.base64.encode(message.pixelData, 0, message.pixelData.length) : options.bytes === Array ? Array.prototype.slice.call(message.pixelData) : message.pixelData;
                return object;
            };

            /**
             * Converts this AtlasEntry to JSON.
             * @function toJSON
             * @memberof zed.scene.AtlasEntry
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            AtlasEntry.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for AtlasEntry
             * @function getTypeUrl
             * @memberof zed.scene.AtlasEntry
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            AtlasEntry.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.AtlasEntry";
            };

            return AtlasEntry;
        })();

        scene.Point = (function() {

            /**
             * Properties of a Point.
             * @memberof zed.scene
             * @interface IPoint
             * @property {number|null} [x] Point x
             * @property {number|null} [y] Point y
             */

            /**
             * Constructs a new Point.
             * @memberof zed.scene
             * @classdesc Represents a Point.
             * @implements IPoint
             * @constructor
             * @param {zed.scene.IPoint=} [properties] Properties to set
             */
            function Point(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Point x.
             * @member {number} x
             * @memberof zed.scene.Point
             * @instance
             */
            Point.prototype.x = 0;

            /**
             * Point y.
             * @member {number} y
             * @memberof zed.scene.Point
             * @instance
             */
            Point.prototype.y = 0;

            /**
             * Creates a new Point instance using the specified properties.
             * @function create
             * @memberof zed.scene.Point
             * @static
             * @param {zed.scene.IPoint=} [properties] Properties to set
             * @returns {zed.scene.Point} Point instance
             */
            Point.create = function create(properties) {
                return new Point(properties);
            };

            /**
             * Encodes the specified Point message. Does not implicitly {@link zed.scene.Point.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Point
             * @static
             * @param {zed.scene.IPoint} message Point message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Point.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.x != null && Object.hasOwnProperty.call(message, "x"))
                    writer.uint32(/* id 1, wireType 5 =*/13).float(message.x);
                if (message.y != null && Object.hasOwnProperty.call(message, "y"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.y);
                return writer;
            };

            /**
             * Encodes the specified Point message, length delimited. Does not implicitly {@link zed.scene.Point.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Point
             * @static
             * @param {zed.scene.IPoint} message Point message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Point.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Point message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Point
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Point} Point
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Point.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Point();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.x = reader.float();
                            break;
                        }
                    case 2: {
                            message.y = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Point message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Point
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Point} Point
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Point.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Point message.
             * @function verify
             * @memberof zed.scene.Point
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Point.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.x != null && message.hasOwnProperty("x"))
                    if (typeof message.x !== "number")
                        return "x: number expected";
                if (message.y != null && message.hasOwnProperty("y"))
                    if (typeof message.y !== "number")
                        return "y: number expected";
                return null;
            };

            /**
             * Creates a Point message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Point
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Point} Point
             */
            Point.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Point)
                    return object;
                let message = new $root.zed.scene.Point();
                if (object.x != null)
                    message.x = Number(object.x);
                if (object.y != null)
                    message.y = Number(object.y);
                return message;
            };

            /**
             * Creates a plain object from a Point message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Point
             * @static
             * @param {zed.scene.Point} message Point
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Point.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.x = 0;
                    object.y = 0;
                }
                if (message.x != null && message.hasOwnProperty("x"))
                    object.x = options.json && !isFinite(message.x) ? String(message.x) : message.x;
                if (message.y != null && message.hasOwnProperty("y"))
                    object.y = options.json && !isFinite(message.y) ? String(message.y) : message.y;
                return object;
            };

            /**
             * Converts this Point to JSON.
             * @function toJSON
             * @memberof zed.scene.Point
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Point.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Point
             * @function getTypeUrl
             * @memberof zed.scene.Point
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Point.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Point";
            };

            return Point;
        })();

        scene.Size = (function() {

            /**
             * Properties of a Size.
             * @memberof zed.scene
             * @interface ISize
             * @property {number|null} [width] Size width
             * @property {number|null} [height] Size height
             */

            /**
             * Constructs a new Size.
             * @memberof zed.scene
             * @classdesc Represents a Size.
             * @implements ISize
             * @constructor
             * @param {zed.scene.ISize=} [properties] Properties to set
             */
            function Size(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Size width.
             * @member {number} width
             * @memberof zed.scene.Size
             * @instance
             */
            Size.prototype.width = 0;

            /**
             * Size height.
             * @member {number} height
             * @memberof zed.scene.Size
             * @instance
             */
            Size.prototype.height = 0;

            /**
             * Creates a new Size instance using the specified properties.
             * @function create
             * @memberof zed.scene.Size
             * @static
             * @param {zed.scene.ISize=} [properties] Properties to set
             * @returns {zed.scene.Size} Size instance
             */
            Size.create = function create(properties) {
                return new Size(properties);
            };

            /**
             * Encodes the specified Size message. Does not implicitly {@link zed.scene.Size.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Size
             * @static
             * @param {zed.scene.ISize} message Size message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Size.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.width != null && Object.hasOwnProperty.call(message, "width"))
                    writer.uint32(/* id 1, wireType 5 =*/13).float(message.width);
                if (message.height != null && Object.hasOwnProperty.call(message, "height"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.height);
                return writer;
            };

            /**
             * Encodes the specified Size message, length delimited. Does not implicitly {@link zed.scene.Size.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Size
             * @static
             * @param {zed.scene.ISize} message Size message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Size.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Size message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Size
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Size} Size
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Size.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Size();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.width = reader.float();
                            break;
                        }
                    case 2: {
                            message.height = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Size message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Size
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Size} Size
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Size.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Size message.
             * @function verify
             * @memberof zed.scene.Size
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Size.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.width != null && message.hasOwnProperty("width"))
                    if (typeof message.width !== "number")
                        return "width: number expected";
                if (message.height != null && message.hasOwnProperty("height"))
                    if (typeof message.height !== "number")
                        return "height: number expected";
                return null;
            };

            /**
             * Creates a Size message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Size
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Size} Size
             */
            Size.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Size)
                    return object;
                let message = new $root.zed.scene.Size();
                if (object.width != null)
                    message.width = Number(object.width);
                if (object.height != null)
                    message.height = Number(object.height);
                return message;
            };

            /**
             * Creates a plain object from a Size message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Size
             * @static
             * @param {zed.scene.Size} message Size
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Size.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.width = 0;
                    object.height = 0;
                }
                if (message.width != null && message.hasOwnProperty("width"))
                    object.width = options.json && !isFinite(message.width) ? String(message.width) : message.width;
                if (message.height != null && message.hasOwnProperty("height"))
                    object.height = options.json && !isFinite(message.height) ? String(message.height) : message.height;
                return object;
            };

            /**
             * Converts this Size to JSON.
             * @function toJSON
             * @memberof zed.scene.Size
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Size.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Size
             * @function getTypeUrl
             * @memberof zed.scene.Size
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Size.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Size";
            };

            return Size;
        })();

        scene.Bounds = (function() {

            /**
             * Properties of a Bounds.
             * @memberof zed.scene
             * @interface IBounds
             * @property {zed.scene.IPoint|null} [origin] Bounds origin
             * @property {zed.scene.ISize|null} [size] Bounds size
             */

            /**
             * Constructs a new Bounds.
             * @memberof zed.scene
             * @classdesc Represents a Bounds.
             * @implements IBounds
             * @constructor
             * @param {zed.scene.IBounds=} [properties] Properties to set
             */
            function Bounds(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Bounds origin.
             * @member {zed.scene.IPoint|null|undefined} origin
             * @memberof zed.scene.Bounds
             * @instance
             */
            Bounds.prototype.origin = null;

            /**
             * Bounds size.
             * @member {zed.scene.ISize|null|undefined} size
             * @memberof zed.scene.Bounds
             * @instance
             */
            Bounds.prototype.size = null;

            /**
             * Creates a new Bounds instance using the specified properties.
             * @function create
             * @memberof zed.scene.Bounds
             * @static
             * @param {zed.scene.IBounds=} [properties] Properties to set
             * @returns {zed.scene.Bounds} Bounds instance
             */
            Bounds.create = function create(properties) {
                return new Bounds(properties);
            };

            /**
             * Encodes the specified Bounds message. Does not implicitly {@link zed.scene.Bounds.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Bounds
             * @static
             * @param {zed.scene.IBounds} message Bounds message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Bounds.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.origin != null && Object.hasOwnProperty.call(message, "origin"))
                    $root.zed.scene.Point.encode(message.origin, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.size != null && Object.hasOwnProperty.call(message, "size"))
                    $root.zed.scene.Size.encode(message.size, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified Bounds message, length delimited. Does not implicitly {@link zed.scene.Bounds.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Bounds
             * @static
             * @param {zed.scene.IBounds} message Bounds message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Bounds.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Bounds message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Bounds
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Bounds} Bounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Bounds.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Bounds();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.origin = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.size = $root.zed.scene.Size.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Bounds message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Bounds
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Bounds} Bounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Bounds.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Bounds message.
             * @function verify
             * @memberof zed.scene.Bounds
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Bounds.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.origin != null && message.hasOwnProperty("origin")) {
                    let error = $root.zed.scene.Point.verify(message.origin);
                    if (error)
                        return "origin." + error;
                }
                if (message.size != null && message.hasOwnProperty("size")) {
                    let error = $root.zed.scene.Size.verify(message.size);
                    if (error)
                        return "size." + error;
                }
                return null;
            };

            /**
             * Creates a Bounds message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Bounds
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Bounds} Bounds
             */
            Bounds.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Bounds)
                    return object;
                let message = new $root.zed.scene.Bounds();
                if (object.origin != null) {
                    if (typeof object.origin !== "object")
                        throw TypeError(".zed.scene.Bounds.origin: object expected");
                    message.origin = $root.zed.scene.Point.fromObject(object.origin);
                }
                if (object.size != null) {
                    if (typeof object.size !== "object")
                        throw TypeError(".zed.scene.Bounds.size: object expected");
                    message.size = $root.zed.scene.Size.fromObject(object.size);
                }
                return message;
            };

            /**
             * Creates a plain object from a Bounds message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Bounds
             * @static
             * @param {zed.scene.Bounds} message Bounds
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Bounds.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.origin = null;
                    object.size = null;
                }
                if (message.origin != null && message.hasOwnProperty("origin"))
                    object.origin = $root.zed.scene.Point.toObject(message.origin, options);
                if (message.size != null && message.hasOwnProperty("size"))
                    object.size = $root.zed.scene.Size.toObject(message.size, options);
                return object;
            };

            /**
             * Converts this Bounds to JSON.
             * @function toJSON
             * @memberof zed.scene.Bounds
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Bounds.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Bounds
             * @function getTypeUrl
             * @memberof zed.scene.Bounds
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Bounds.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Bounds";
            };

            return Bounds;
        })();

        scene.ContentMask = (function() {

            /**
             * Properties of a ContentMask.
             * @memberof zed.scene
             * @interface IContentMask
             * @property {zed.scene.IBounds|null} [bounds] ContentMask bounds
             */

            /**
             * Constructs a new ContentMask.
             * @memberof zed.scene
             * @classdesc Represents a ContentMask.
             * @implements IContentMask
             * @constructor
             * @param {zed.scene.IContentMask=} [properties] Properties to set
             */
            function ContentMask(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ContentMask bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.ContentMask
             * @instance
             */
            ContentMask.prototype.bounds = null;

            /**
             * Creates a new ContentMask instance using the specified properties.
             * @function create
             * @memberof zed.scene.ContentMask
             * @static
             * @param {zed.scene.IContentMask=} [properties] Properties to set
             * @returns {zed.scene.ContentMask} ContentMask instance
             */
            ContentMask.create = function create(properties) {
                return new ContentMask(properties);
            };

            /**
             * Encodes the specified ContentMask message. Does not implicitly {@link zed.scene.ContentMask.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.ContentMask
             * @static
             * @param {zed.scene.IContentMask} message ContentMask message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ContentMask.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified ContentMask message, length delimited. Does not implicitly {@link zed.scene.ContentMask.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.ContentMask
             * @static
             * @param {zed.scene.IContentMask} message ContentMask message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ContentMask.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a ContentMask message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.ContentMask
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.ContentMask} ContentMask
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ContentMask.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.ContentMask();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a ContentMask message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.ContentMask
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.ContentMask} ContentMask
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ContentMask.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a ContentMask message.
             * @function verify
             * @memberof zed.scene.ContentMask
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ContentMask.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                return null;
            };

            /**
             * Creates a ContentMask message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.ContentMask
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.ContentMask} ContentMask
             */
            ContentMask.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.ContentMask)
                    return object;
                let message = new $root.zed.scene.ContentMask();
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.ContentMask.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                return message;
            };

            /**
             * Creates a plain object from a ContentMask message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.ContentMask
             * @static
             * @param {zed.scene.ContentMask} message ContentMask
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ContentMask.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults)
                    object.bounds = null;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                return object;
            };

            /**
             * Converts this ContentMask to JSON.
             * @function toJSON
             * @memberof zed.scene.ContentMask
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ContentMask.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for ContentMask
             * @function getTypeUrl
             * @memberof zed.scene.ContentMask
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            ContentMask.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.ContentMask";
            };

            return ContentMask;
        })();

        scene.Corners = (function() {

            /**
             * Properties of a Corners.
             * @memberof zed.scene
             * @interface ICorners
             * @property {number|null} [topLeft] Corners topLeft
             * @property {number|null} [topRight] Corners topRight
             * @property {number|null} [bottomRight] Corners bottomRight
             * @property {number|null} [bottomLeft] Corners bottomLeft
             */

            /**
             * Constructs a new Corners.
             * @memberof zed.scene
             * @classdesc Represents a Corners.
             * @implements ICorners
             * @constructor
             * @param {zed.scene.ICorners=} [properties] Properties to set
             */
            function Corners(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Corners topLeft.
             * @member {number} topLeft
             * @memberof zed.scene.Corners
             * @instance
             */
            Corners.prototype.topLeft = 0;

            /**
             * Corners topRight.
             * @member {number} topRight
             * @memberof zed.scene.Corners
             * @instance
             */
            Corners.prototype.topRight = 0;

            /**
             * Corners bottomRight.
             * @member {number} bottomRight
             * @memberof zed.scene.Corners
             * @instance
             */
            Corners.prototype.bottomRight = 0;

            /**
             * Corners bottomLeft.
             * @member {number} bottomLeft
             * @memberof zed.scene.Corners
             * @instance
             */
            Corners.prototype.bottomLeft = 0;

            /**
             * Creates a new Corners instance using the specified properties.
             * @function create
             * @memberof zed.scene.Corners
             * @static
             * @param {zed.scene.ICorners=} [properties] Properties to set
             * @returns {zed.scene.Corners} Corners instance
             */
            Corners.create = function create(properties) {
                return new Corners(properties);
            };

            /**
             * Encodes the specified Corners message. Does not implicitly {@link zed.scene.Corners.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Corners
             * @static
             * @param {zed.scene.ICorners} message Corners message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Corners.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.topLeft != null && Object.hasOwnProperty.call(message, "topLeft"))
                    writer.uint32(/* id 1, wireType 5 =*/13).float(message.topLeft);
                if (message.topRight != null && Object.hasOwnProperty.call(message, "topRight"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.topRight);
                if (message.bottomRight != null && Object.hasOwnProperty.call(message, "bottomRight"))
                    writer.uint32(/* id 3, wireType 5 =*/29).float(message.bottomRight);
                if (message.bottomLeft != null && Object.hasOwnProperty.call(message, "bottomLeft"))
                    writer.uint32(/* id 4, wireType 5 =*/37).float(message.bottomLeft);
                return writer;
            };

            /**
             * Encodes the specified Corners message, length delimited. Does not implicitly {@link zed.scene.Corners.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Corners
             * @static
             * @param {zed.scene.ICorners} message Corners message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Corners.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Corners message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Corners
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Corners} Corners
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Corners.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Corners();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.topLeft = reader.float();
                            break;
                        }
                    case 2: {
                            message.topRight = reader.float();
                            break;
                        }
                    case 3: {
                            message.bottomRight = reader.float();
                            break;
                        }
                    case 4: {
                            message.bottomLeft = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Corners message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Corners
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Corners} Corners
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Corners.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Corners message.
             * @function verify
             * @memberof zed.scene.Corners
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Corners.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.topLeft != null && message.hasOwnProperty("topLeft"))
                    if (typeof message.topLeft !== "number")
                        return "topLeft: number expected";
                if (message.topRight != null && message.hasOwnProperty("topRight"))
                    if (typeof message.topRight !== "number")
                        return "topRight: number expected";
                if (message.bottomRight != null && message.hasOwnProperty("bottomRight"))
                    if (typeof message.bottomRight !== "number")
                        return "bottomRight: number expected";
                if (message.bottomLeft != null && message.hasOwnProperty("bottomLeft"))
                    if (typeof message.bottomLeft !== "number")
                        return "bottomLeft: number expected";
                return null;
            };

            /**
             * Creates a Corners message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Corners
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Corners} Corners
             */
            Corners.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Corners)
                    return object;
                let message = new $root.zed.scene.Corners();
                if (object.topLeft != null)
                    message.topLeft = Number(object.topLeft);
                if (object.topRight != null)
                    message.topRight = Number(object.topRight);
                if (object.bottomRight != null)
                    message.bottomRight = Number(object.bottomRight);
                if (object.bottomLeft != null)
                    message.bottomLeft = Number(object.bottomLeft);
                return message;
            };

            /**
             * Creates a plain object from a Corners message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Corners
             * @static
             * @param {zed.scene.Corners} message Corners
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Corners.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.topLeft = 0;
                    object.topRight = 0;
                    object.bottomRight = 0;
                    object.bottomLeft = 0;
                }
                if (message.topLeft != null && message.hasOwnProperty("topLeft"))
                    object.topLeft = options.json && !isFinite(message.topLeft) ? String(message.topLeft) : message.topLeft;
                if (message.topRight != null && message.hasOwnProperty("topRight"))
                    object.topRight = options.json && !isFinite(message.topRight) ? String(message.topRight) : message.topRight;
                if (message.bottomRight != null && message.hasOwnProperty("bottomRight"))
                    object.bottomRight = options.json && !isFinite(message.bottomRight) ? String(message.bottomRight) : message.bottomRight;
                if (message.bottomLeft != null && message.hasOwnProperty("bottomLeft"))
                    object.bottomLeft = options.json && !isFinite(message.bottomLeft) ? String(message.bottomLeft) : message.bottomLeft;
                return object;
            };

            /**
             * Converts this Corners to JSON.
             * @function toJSON
             * @memberof zed.scene.Corners
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Corners.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Corners
             * @function getTypeUrl
             * @memberof zed.scene.Corners
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Corners.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Corners";
            };

            return Corners;
        })();

        scene.Edges = (function() {

            /**
             * Properties of an Edges.
             * @memberof zed.scene
             * @interface IEdges
             * @property {number|null} [top] Edges top
             * @property {number|null} [right] Edges right
             * @property {number|null} [bottom] Edges bottom
             * @property {number|null} [left] Edges left
             */

            /**
             * Constructs a new Edges.
             * @memberof zed.scene
             * @classdesc Represents an Edges.
             * @implements IEdges
             * @constructor
             * @param {zed.scene.IEdges=} [properties] Properties to set
             */
            function Edges(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Edges top.
             * @member {number} top
             * @memberof zed.scene.Edges
             * @instance
             */
            Edges.prototype.top = 0;

            /**
             * Edges right.
             * @member {number} right
             * @memberof zed.scene.Edges
             * @instance
             */
            Edges.prototype.right = 0;

            /**
             * Edges bottom.
             * @member {number} bottom
             * @memberof zed.scene.Edges
             * @instance
             */
            Edges.prototype.bottom = 0;

            /**
             * Edges left.
             * @member {number} left
             * @memberof zed.scene.Edges
             * @instance
             */
            Edges.prototype.left = 0;

            /**
             * Creates a new Edges instance using the specified properties.
             * @function create
             * @memberof zed.scene.Edges
             * @static
             * @param {zed.scene.IEdges=} [properties] Properties to set
             * @returns {zed.scene.Edges} Edges instance
             */
            Edges.create = function create(properties) {
                return new Edges(properties);
            };

            /**
             * Encodes the specified Edges message. Does not implicitly {@link zed.scene.Edges.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Edges
             * @static
             * @param {zed.scene.IEdges} message Edges message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Edges.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.top != null && Object.hasOwnProperty.call(message, "top"))
                    writer.uint32(/* id 1, wireType 5 =*/13).float(message.top);
                if (message.right != null && Object.hasOwnProperty.call(message, "right"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.right);
                if (message.bottom != null && Object.hasOwnProperty.call(message, "bottom"))
                    writer.uint32(/* id 3, wireType 5 =*/29).float(message.bottom);
                if (message.left != null && Object.hasOwnProperty.call(message, "left"))
                    writer.uint32(/* id 4, wireType 5 =*/37).float(message.left);
                return writer;
            };

            /**
             * Encodes the specified Edges message, length delimited. Does not implicitly {@link zed.scene.Edges.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Edges
             * @static
             * @param {zed.scene.IEdges} message Edges message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Edges.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an Edges message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Edges
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Edges} Edges
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Edges.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Edges();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.top = reader.float();
                            break;
                        }
                    case 2: {
                            message.right = reader.float();
                            break;
                        }
                    case 3: {
                            message.bottom = reader.float();
                            break;
                        }
                    case 4: {
                            message.left = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an Edges message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Edges
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Edges} Edges
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Edges.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an Edges message.
             * @function verify
             * @memberof zed.scene.Edges
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Edges.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.top != null && message.hasOwnProperty("top"))
                    if (typeof message.top !== "number")
                        return "top: number expected";
                if (message.right != null && message.hasOwnProperty("right"))
                    if (typeof message.right !== "number")
                        return "right: number expected";
                if (message.bottom != null && message.hasOwnProperty("bottom"))
                    if (typeof message.bottom !== "number")
                        return "bottom: number expected";
                if (message.left != null && message.hasOwnProperty("left"))
                    if (typeof message.left !== "number")
                        return "left: number expected";
                return null;
            };

            /**
             * Creates an Edges message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Edges
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Edges} Edges
             */
            Edges.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Edges)
                    return object;
                let message = new $root.zed.scene.Edges();
                if (object.top != null)
                    message.top = Number(object.top);
                if (object.right != null)
                    message.right = Number(object.right);
                if (object.bottom != null)
                    message.bottom = Number(object.bottom);
                if (object.left != null)
                    message.left = Number(object.left);
                return message;
            };

            /**
             * Creates a plain object from an Edges message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Edges
             * @static
             * @param {zed.scene.Edges} message Edges
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Edges.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.top = 0;
                    object.right = 0;
                    object.bottom = 0;
                    object.left = 0;
                }
                if (message.top != null && message.hasOwnProperty("top"))
                    object.top = options.json && !isFinite(message.top) ? String(message.top) : message.top;
                if (message.right != null && message.hasOwnProperty("right"))
                    object.right = options.json && !isFinite(message.right) ? String(message.right) : message.right;
                if (message.bottom != null && message.hasOwnProperty("bottom"))
                    object.bottom = options.json && !isFinite(message.bottom) ? String(message.bottom) : message.bottom;
                if (message.left != null && message.hasOwnProperty("left"))
                    object.left = options.json && !isFinite(message.left) ? String(message.left) : message.left;
                return object;
            };

            /**
             * Converts this Edges to JSON.
             * @function toJSON
             * @memberof zed.scene.Edges
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Edges.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Edges
             * @function getTypeUrl
             * @memberof zed.scene.Edges
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Edges.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Edges";
            };

            return Edges;
        })();

        scene.Hsla = (function() {

            /**
             * Properties of a Hsla.
             * @memberof zed.scene
             * @interface IHsla
             * @property {number|null} [h] Hsla h
             * @property {number|null} [s] Hsla s
             * @property {number|null} [l] Hsla l
             * @property {number|null} [a] Hsla a
             */

            /**
             * Constructs a new Hsla.
             * @memberof zed.scene
             * @classdesc Represents a Hsla.
             * @implements IHsla
             * @constructor
             * @param {zed.scene.IHsla=} [properties] Properties to set
             */
            function Hsla(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Hsla h.
             * @member {number} h
             * @memberof zed.scene.Hsla
             * @instance
             */
            Hsla.prototype.h = 0;

            /**
             * Hsla s.
             * @member {number} s
             * @memberof zed.scene.Hsla
             * @instance
             */
            Hsla.prototype.s = 0;

            /**
             * Hsla l.
             * @member {number} l
             * @memberof zed.scene.Hsla
             * @instance
             */
            Hsla.prototype.l = 0;

            /**
             * Hsla a.
             * @member {number} a
             * @memberof zed.scene.Hsla
             * @instance
             */
            Hsla.prototype.a = 0;

            /**
             * Creates a new Hsla instance using the specified properties.
             * @function create
             * @memberof zed.scene.Hsla
             * @static
             * @param {zed.scene.IHsla=} [properties] Properties to set
             * @returns {zed.scene.Hsla} Hsla instance
             */
            Hsla.create = function create(properties) {
                return new Hsla(properties);
            };

            /**
             * Encodes the specified Hsla message. Does not implicitly {@link zed.scene.Hsla.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Hsla
             * @static
             * @param {zed.scene.IHsla} message Hsla message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Hsla.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.h != null && Object.hasOwnProperty.call(message, "h"))
                    writer.uint32(/* id 1, wireType 5 =*/13).float(message.h);
                if (message.s != null && Object.hasOwnProperty.call(message, "s"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.s);
                if (message.l != null && Object.hasOwnProperty.call(message, "l"))
                    writer.uint32(/* id 3, wireType 5 =*/29).float(message.l);
                if (message.a != null && Object.hasOwnProperty.call(message, "a"))
                    writer.uint32(/* id 4, wireType 5 =*/37).float(message.a);
                return writer;
            };

            /**
             * Encodes the specified Hsla message, length delimited. Does not implicitly {@link zed.scene.Hsla.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Hsla
             * @static
             * @param {zed.scene.IHsla} message Hsla message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Hsla.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Hsla message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Hsla
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Hsla} Hsla
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Hsla.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Hsla();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.h = reader.float();
                            break;
                        }
                    case 2: {
                            message.s = reader.float();
                            break;
                        }
                    case 3: {
                            message.l = reader.float();
                            break;
                        }
                    case 4: {
                            message.a = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Hsla message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Hsla
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Hsla} Hsla
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Hsla.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Hsla message.
             * @function verify
             * @memberof zed.scene.Hsla
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Hsla.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.h != null && message.hasOwnProperty("h"))
                    if (typeof message.h !== "number")
                        return "h: number expected";
                if (message.s != null && message.hasOwnProperty("s"))
                    if (typeof message.s !== "number")
                        return "s: number expected";
                if (message.l != null && message.hasOwnProperty("l"))
                    if (typeof message.l !== "number")
                        return "l: number expected";
                if (message.a != null && message.hasOwnProperty("a"))
                    if (typeof message.a !== "number")
                        return "a: number expected";
                return null;
            };

            /**
             * Creates a Hsla message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Hsla
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Hsla} Hsla
             */
            Hsla.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Hsla)
                    return object;
                let message = new $root.zed.scene.Hsla();
                if (object.h != null)
                    message.h = Number(object.h);
                if (object.s != null)
                    message.s = Number(object.s);
                if (object.l != null)
                    message.l = Number(object.l);
                if (object.a != null)
                    message.a = Number(object.a);
                return message;
            };

            /**
             * Creates a plain object from a Hsla message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Hsla
             * @static
             * @param {zed.scene.Hsla} message Hsla
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Hsla.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.h = 0;
                    object.s = 0;
                    object.l = 0;
                    object.a = 0;
                }
                if (message.h != null && message.hasOwnProperty("h"))
                    object.h = options.json && !isFinite(message.h) ? String(message.h) : message.h;
                if (message.s != null && message.hasOwnProperty("s"))
                    object.s = options.json && !isFinite(message.s) ? String(message.s) : message.s;
                if (message.l != null && message.hasOwnProperty("l"))
                    object.l = options.json && !isFinite(message.l) ? String(message.l) : message.l;
                if (message.a != null && message.hasOwnProperty("a"))
                    object.a = options.json && !isFinite(message.a) ? String(message.a) : message.a;
                return object;
            };

            /**
             * Converts this Hsla to JSON.
             * @function toJSON
             * @memberof zed.scene.Hsla
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Hsla.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Hsla
             * @function getTypeUrl
             * @memberof zed.scene.Hsla
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Hsla.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Hsla";
            };

            return Hsla;
        })();

        scene.LinearColorStop = (function() {

            /**
             * Properties of a LinearColorStop.
             * @memberof zed.scene
             * @interface ILinearColorStop
             * @property {zed.scene.IHsla|null} [color] LinearColorStop color
             * @property {number|null} [percentage] LinearColorStop percentage
             */

            /**
             * Constructs a new LinearColorStop.
             * @memberof zed.scene
             * @classdesc Represents a LinearColorStop.
             * @implements ILinearColorStop
             * @constructor
             * @param {zed.scene.ILinearColorStop=} [properties] Properties to set
             */
            function LinearColorStop(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * LinearColorStop color.
             * @member {zed.scene.IHsla|null|undefined} color
             * @memberof zed.scene.LinearColorStop
             * @instance
             */
            LinearColorStop.prototype.color = null;

            /**
             * LinearColorStop percentage.
             * @member {number} percentage
             * @memberof zed.scene.LinearColorStop
             * @instance
             */
            LinearColorStop.prototype.percentage = 0;

            /**
             * Creates a new LinearColorStop instance using the specified properties.
             * @function create
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {zed.scene.ILinearColorStop=} [properties] Properties to set
             * @returns {zed.scene.LinearColorStop} LinearColorStop instance
             */
            LinearColorStop.create = function create(properties) {
                return new LinearColorStop(properties);
            };

            /**
             * Encodes the specified LinearColorStop message. Does not implicitly {@link zed.scene.LinearColorStop.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {zed.scene.ILinearColorStop} message LinearColorStop message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            LinearColorStop.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.color != null && Object.hasOwnProperty.call(message, "color"))
                    $root.zed.scene.Hsla.encode(message.color, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.percentage != null && Object.hasOwnProperty.call(message, "percentage"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.percentage);
                return writer;
            };

            /**
             * Encodes the specified LinearColorStop message, length delimited. Does not implicitly {@link zed.scene.LinearColorStop.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {zed.scene.ILinearColorStop} message LinearColorStop message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            LinearColorStop.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a LinearColorStop message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.LinearColorStop} LinearColorStop
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            LinearColorStop.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.LinearColorStop();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.color = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.percentage = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a LinearColorStop message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.LinearColorStop} LinearColorStop
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            LinearColorStop.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a LinearColorStop message.
             * @function verify
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            LinearColorStop.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.color != null && message.hasOwnProperty("color")) {
                    let error = $root.zed.scene.Hsla.verify(message.color);
                    if (error)
                        return "color." + error;
                }
                if (message.percentage != null && message.hasOwnProperty("percentage"))
                    if (typeof message.percentage !== "number")
                        return "percentage: number expected";
                return null;
            };

            /**
             * Creates a LinearColorStop message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.LinearColorStop} LinearColorStop
             */
            LinearColorStop.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.LinearColorStop)
                    return object;
                let message = new $root.zed.scene.LinearColorStop();
                if (object.color != null) {
                    if (typeof object.color !== "object")
                        throw TypeError(".zed.scene.LinearColorStop.color: object expected");
                    message.color = $root.zed.scene.Hsla.fromObject(object.color);
                }
                if (object.percentage != null)
                    message.percentage = Number(object.percentage);
                return message;
            };

            /**
             * Creates a plain object from a LinearColorStop message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {zed.scene.LinearColorStop} message LinearColorStop
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            LinearColorStop.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.color = null;
                    object.percentage = 0;
                }
                if (message.color != null && message.hasOwnProperty("color"))
                    object.color = $root.zed.scene.Hsla.toObject(message.color, options);
                if (message.percentage != null && message.hasOwnProperty("percentage"))
                    object.percentage = options.json && !isFinite(message.percentage) ? String(message.percentage) : message.percentage;
                return object;
            };

            /**
             * Converts this LinearColorStop to JSON.
             * @function toJSON
             * @memberof zed.scene.LinearColorStop
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            LinearColorStop.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for LinearColorStop
             * @function getTypeUrl
             * @memberof zed.scene.LinearColorStop
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            LinearColorStop.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.LinearColorStop";
            };

            return LinearColorStop;
        })();

        scene.Background = (function() {

            /**
             * Properties of a Background.
             * @memberof zed.scene
             * @interface IBackground
             * @property {number|null} [tag] Background tag
             * @property {number|null} [colorSpace] Background colorSpace
             * @property {zed.scene.IHsla|null} [solid] Background solid
             * @property {number|null} [gradientAngleOrPatternHeight] Background gradientAngleOrPatternHeight
             * @property {Array.<zed.scene.ILinearColorStop>|null} [colors] Background colors
             */

            /**
             * Constructs a new Background.
             * @memberof zed.scene
             * @classdesc Represents a Background.
             * @implements IBackground
             * @constructor
             * @param {zed.scene.IBackground=} [properties] Properties to set
             */
            function Background(properties) {
                this.colors = [];
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Background tag.
             * @member {number} tag
             * @memberof zed.scene.Background
             * @instance
             */
            Background.prototype.tag = 0;

            /**
             * Background colorSpace.
             * @member {number} colorSpace
             * @memberof zed.scene.Background
             * @instance
             */
            Background.prototype.colorSpace = 0;

            /**
             * Background solid.
             * @member {zed.scene.IHsla|null|undefined} solid
             * @memberof zed.scene.Background
             * @instance
             */
            Background.prototype.solid = null;

            /**
             * Background gradientAngleOrPatternHeight.
             * @member {number} gradientAngleOrPatternHeight
             * @memberof zed.scene.Background
             * @instance
             */
            Background.prototype.gradientAngleOrPatternHeight = 0;

            /**
             * Background colors.
             * @member {Array.<zed.scene.ILinearColorStop>} colors
             * @memberof zed.scene.Background
             * @instance
             */
            Background.prototype.colors = $util.emptyArray;

            /**
             * Creates a new Background instance using the specified properties.
             * @function create
             * @memberof zed.scene.Background
             * @static
             * @param {zed.scene.IBackground=} [properties] Properties to set
             * @returns {zed.scene.Background} Background instance
             */
            Background.create = function create(properties) {
                return new Background(properties);
            };

            /**
             * Encodes the specified Background message. Does not implicitly {@link zed.scene.Background.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Background
             * @static
             * @param {zed.scene.IBackground} message Background message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Background.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.tag != null && Object.hasOwnProperty.call(message, "tag"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.tag);
                if (message.colorSpace != null && Object.hasOwnProperty.call(message, "colorSpace"))
                    writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.colorSpace);
                if (message.solid != null && Object.hasOwnProperty.call(message, "solid"))
                    $root.zed.scene.Hsla.encode(message.solid, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.gradientAngleOrPatternHeight != null && Object.hasOwnProperty.call(message, "gradientAngleOrPatternHeight"))
                    writer.uint32(/* id 4, wireType 5 =*/37).float(message.gradientAngleOrPatternHeight);
                if (message.colors != null && message.colors.length)
                    for (let i = 0; i < message.colors.length; ++i)
                        $root.zed.scene.LinearColorStop.encode(message.colors[i], writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified Background message, length delimited. Does not implicitly {@link zed.scene.Background.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Background
             * @static
             * @param {zed.scene.IBackground} message Background message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Background.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Background message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Background
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Background} Background
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Background.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Background();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.tag = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.colorSpace = reader.uint32();
                            break;
                        }
                    case 3: {
                            message.solid = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.gradientAngleOrPatternHeight = reader.float();
                            break;
                        }
                    case 5: {
                            if (!(message.colors && message.colors.length))
                                message.colors = [];
                            message.colors.push($root.zed.scene.LinearColorStop.decode(reader, reader.uint32()));
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Background message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Background
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Background} Background
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Background.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Background message.
             * @function verify
             * @memberof zed.scene.Background
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Background.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.tag != null && message.hasOwnProperty("tag"))
                    if (!$util.isInteger(message.tag))
                        return "tag: integer expected";
                if (message.colorSpace != null && message.hasOwnProperty("colorSpace"))
                    if (!$util.isInteger(message.colorSpace))
                        return "colorSpace: integer expected";
                if (message.solid != null && message.hasOwnProperty("solid")) {
                    let error = $root.zed.scene.Hsla.verify(message.solid);
                    if (error)
                        return "solid." + error;
                }
                if (message.gradientAngleOrPatternHeight != null && message.hasOwnProperty("gradientAngleOrPatternHeight"))
                    if (typeof message.gradientAngleOrPatternHeight !== "number")
                        return "gradientAngleOrPatternHeight: number expected";
                if (message.colors != null && message.hasOwnProperty("colors")) {
                    if (!Array.isArray(message.colors))
                        return "colors: array expected";
                    for (let i = 0; i < message.colors.length; ++i) {
                        let error = $root.zed.scene.LinearColorStop.verify(message.colors[i]);
                        if (error)
                            return "colors." + error;
                    }
                }
                return null;
            };

            /**
             * Creates a Background message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Background
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Background} Background
             */
            Background.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Background)
                    return object;
                let message = new $root.zed.scene.Background();
                if (object.tag != null)
                    message.tag = object.tag >>> 0;
                if (object.colorSpace != null)
                    message.colorSpace = object.colorSpace >>> 0;
                if (object.solid != null) {
                    if (typeof object.solid !== "object")
                        throw TypeError(".zed.scene.Background.solid: object expected");
                    message.solid = $root.zed.scene.Hsla.fromObject(object.solid);
                }
                if (object.gradientAngleOrPatternHeight != null)
                    message.gradientAngleOrPatternHeight = Number(object.gradientAngleOrPatternHeight);
                if (object.colors) {
                    if (!Array.isArray(object.colors))
                        throw TypeError(".zed.scene.Background.colors: array expected");
                    message.colors = [];
                    for (let i = 0; i < object.colors.length; ++i) {
                        if (typeof object.colors[i] !== "object")
                            throw TypeError(".zed.scene.Background.colors: object expected");
                        message.colors[i] = $root.zed.scene.LinearColorStop.fromObject(object.colors[i]);
                    }
                }
                return message;
            };

            /**
             * Creates a plain object from a Background message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Background
             * @static
             * @param {zed.scene.Background} message Background
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Background.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.arrays || options.defaults)
                    object.colors = [];
                if (options.defaults) {
                    object.tag = 0;
                    object.colorSpace = 0;
                    object.solid = null;
                    object.gradientAngleOrPatternHeight = 0;
                }
                if (message.tag != null && message.hasOwnProperty("tag"))
                    object.tag = message.tag;
                if (message.colorSpace != null && message.hasOwnProperty("colorSpace"))
                    object.colorSpace = message.colorSpace;
                if (message.solid != null && message.hasOwnProperty("solid"))
                    object.solid = $root.zed.scene.Hsla.toObject(message.solid, options);
                if (message.gradientAngleOrPatternHeight != null && message.hasOwnProperty("gradientAngleOrPatternHeight"))
                    object.gradientAngleOrPatternHeight = options.json && !isFinite(message.gradientAngleOrPatternHeight) ? String(message.gradientAngleOrPatternHeight) : message.gradientAngleOrPatternHeight;
                if (message.colors && message.colors.length) {
                    object.colors = [];
                    for (let j = 0; j < message.colors.length; ++j)
                        object.colors[j] = $root.zed.scene.LinearColorStop.toObject(message.colors[j], options);
                }
                return object;
            };

            /**
             * Converts this Background to JSON.
             * @function toJSON
             * @memberof zed.scene.Background
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Background.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Background
             * @function getTypeUrl
             * @memberof zed.scene.Background
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Background.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Background";
            };

            return Background;
        })();

        scene.AtlasTextureId = (function() {

            /**
             * Properties of an AtlasTextureId.
             * @memberof zed.scene
             * @interface IAtlasTextureId
             * @property {number|null} [index] AtlasTextureId index
             * @property {number|null} [kind] AtlasTextureId kind
             */

            /**
             * Constructs a new AtlasTextureId.
             * @memberof zed.scene
             * @classdesc Represents an AtlasTextureId.
             * @implements IAtlasTextureId
             * @constructor
             * @param {zed.scene.IAtlasTextureId=} [properties] Properties to set
             */
            function AtlasTextureId(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * AtlasTextureId index.
             * @member {number} index
             * @memberof zed.scene.AtlasTextureId
             * @instance
             */
            AtlasTextureId.prototype.index = 0;

            /**
             * AtlasTextureId kind.
             * @member {number} kind
             * @memberof zed.scene.AtlasTextureId
             * @instance
             */
            AtlasTextureId.prototype.kind = 0;

            /**
             * Creates a new AtlasTextureId instance using the specified properties.
             * @function create
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {zed.scene.IAtlasTextureId=} [properties] Properties to set
             * @returns {zed.scene.AtlasTextureId} AtlasTextureId instance
             */
            AtlasTextureId.create = function create(properties) {
                return new AtlasTextureId(properties);
            };

            /**
             * Encodes the specified AtlasTextureId message. Does not implicitly {@link zed.scene.AtlasTextureId.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {zed.scene.IAtlasTextureId} message AtlasTextureId message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasTextureId.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.index != null && Object.hasOwnProperty.call(message, "index"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.index);
                if (message.kind != null && Object.hasOwnProperty.call(message, "kind"))
                    writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.kind);
                return writer;
            };

            /**
             * Encodes the specified AtlasTextureId message, length delimited. Does not implicitly {@link zed.scene.AtlasTextureId.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {zed.scene.IAtlasTextureId} message AtlasTextureId message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasTextureId.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an AtlasTextureId message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.AtlasTextureId} AtlasTextureId
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasTextureId.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.AtlasTextureId();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.index = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.kind = reader.uint32();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an AtlasTextureId message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.AtlasTextureId} AtlasTextureId
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasTextureId.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an AtlasTextureId message.
             * @function verify
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            AtlasTextureId.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.index != null && message.hasOwnProperty("index"))
                    if (!$util.isInteger(message.index))
                        return "index: integer expected";
                if (message.kind != null && message.hasOwnProperty("kind"))
                    if (!$util.isInteger(message.kind))
                        return "kind: integer expected";
                return null;
            };

            /**
             * Creates an AtlasTextureId message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.AtlasTextureId} AtlasTextureId
             */
            AtlasTextureId.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.AtlasTextureId)
                    return object;
                let message = new $root.zed.scene.AtlasTextureId();
                if (object.index != null)
                    message.index = object.index >>> 0;
                if (object.kind != null)
                    message.kind = object.kind >>> 0;
                return message;
            };

            /**
             * Creates a plain object from an AtlasTextureId message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {zed.scene.AtlasTextureId} message AtlasTextureId
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            AtlasTextureId.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.index = 0;
                    object.kind = 0;
                }
                if (message.index != null && message.hasOwnProperty("index"))
                    object.index = message.index;
                if (message.kind != null && message.hasOwnProperty("kind"))
                    object.kind = message.kind;
                return object;
            };

            /**
             * Converts this AtlasTextureId to JSON.
             * @function toJSON
             * @memberof zed.scene.AtlasTextureId
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            AtlasTextureId.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for AtlasTextureId
             * @function getTypeUrl
             * @memberof zed.scene.AtlasTextureId
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            AtlasTextureId.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.AtlasTextureId";
            };

            return AtlasTextureId;
        })();

        scene.AtlasBounds = (function() {

            /**
             * Properties of an AtlasBounds.
             * @memberof zed.scene
             * @interface IAtlasBounds
             * @property {number|null} [originX] AtlasBounds originX
             * @property {number|null} [originY] AtlasBounds originY
             * @property {number|null} [width] AtlasBounds width
             * @property {number|null} [height] AtlasBounds height
             */

            /**
             * Constructs a new AtlasBounds.
             * @memberof zed.scene
             * @classdesc Represents an AtlasBounds.
             * @implements IAtlasBounds
             * @constructor
             * @param {zed.scene.IAtlasBounds=} [properties] Properties to set
             */
            function AtlasBounds(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * AtlasBounds originX.
             * @member {number} originX
             * @memberof zed.scene.AtlasBounds
             * @instance
             */
            AtlasBounds.prototype.originX = 0;

            /**
             * AtlasBounds originY.
             * @member {number} originY
             * @memberof zed.scene.AtlasBounds
             * @instance
             */
            AtlasBounds.prototype.originY = 0;

            /**
             * AtlasBounds width.
             * @member {number} width
             * @memberof zed.scene.AtlasBounds
             * @instance
             */
            AtlasBounds.prototype.width = 0;

            /**
             * AtlasBounds height.
             * @member {number} height
             * @memberof zed.scene.AtlasBounds
             * @instance
             */
            AtlasBounds.prototype.height = 0;

            /**
             * Creates a new AtlasBounds instance using the specified properties.
             * @function create
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {zed.scene.IAtlasBounds=} [properties] Properties to set
             * @returns {zed.scene.AtlasBounds} AtlasBounds instance
             */
            AtlasBounds.create = function create(properties) {
                return new AtlasBounds(properties);
            };

            /**
             * Encodes the specified AtlasBounds message. Does not implicitly {@link zed.scene.AtlasBounds.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {zed.scene.IAtlasBounds} message AtlasBounds message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasBounds.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.originX != null && Object.hasOwnProperty.call(message, "originX"))
                    writer.uint32(/* id 1, wireType 0 =*/8).int32(message.originX);
                if (message.originY != null && Object.hasOwnProperty.call(message, "originY"))
                    writer.uint32(/* id 2, wireType 0 =*/16).int32(message.originY);
                if (message.width != null && Object.hasOwnProperty.call(message, "width"))
                    writer.uint32(/* id 3, wireType 0 =*/24).int32(message.width);
                if (message.height != null && Object.hasOwnProperty.call(message, "height"))
                    writer.uint32(/* id 4, wireType 0 =*/32).int32(message.height);
                return writer;
            };

            /**
             * Encodes the specified AtlasBounds message, length delimited. Does not implicitly {@link zed.scene.AtlasBounds.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {zed.scene.IAtlasBounds} message AtlasBounds message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasBounds.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an AtlasBounds message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.AtlasBounds} AtlasBounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasBounds.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.AtlasBounds();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.originX = reader.int32();
                            break;
                        }
                    case 2: {
                            message.originY = reader.int32();
                            break;
                        }
                    case 3: {
                            message.width = reader.int32();
                            break;
                        }
                    case 4: {
                            message.height = reader.int32();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an AtlasBounds message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.AtlasBounds} AtlasBounds
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasBounds.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an AtlasBounds message.
             * @function verify
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            AtlasBounds.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.originX != null && message.hasOwnProperty("originX"))
                    if (!$util.isInteger(message.originX))
                        return "originX: integer expected";
                if (message.originY != null && message.hasOwnProperty("originY"))
                    if (!$util.isInteger(message.originY))
                        return "originY: integer expected";
                if (message.width != null && message.hasOwnProperty("width"))
                    if (!$util.isInteger(message.width))
                        return "width: integer expected";
                if (message.height != null && message.hasOwnProperty("height"))
                    if (!$util.isInteger(message.height))
                        return "height: integer expected";
                return null;
            };

            /**
             * Creates an AtlasBounds message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.AtlasBounds} AtlasBounds
             */
            AtlasBounds.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.AtlasBounds)
                    return object;
                let message = new $root.zed.scene.AtlasBounds();
                if (object.originX != null)
                    message.originX = object.originX | 0;
                if (object.originY != null)
                    message.originY = object.originY | 0;
                if (object.width != null)
                    message.width = object.width | 0;
                if (object.height != null)
                    message.height = object.height | 0;
                return message;
            };

            /**
             * Creates a plain object from an AtlasBounds message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {zed.scene.AtlasBounds} message AtlasBounds
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            AtlasBounds.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.originX = 0;
                    object.originY = 0;
                    object.width = 0;
                    object.height = 0;
                }
                if (message.originX != null && message.hasOwnProperty("originX"))
                    object.originX = message.originX;
                if (message.originY != null && message.hasOwnProperty("originY"))
                    object.originY = message.originY;
                if (message.width != null && message.hasOwnProperty("width"))
                    object.width = message.width;
                if (message.height != null && message.hasOwnProperty("height"))
                    object.height = message.height;
                return object;
            };

            /**
             * Converts this AtlasBounds to JSON.
             * @function toJSON
             * @memberof zed.scene.AtlasBounds
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            AtlasBounds.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for AtlasBounds
             * @function getTypeUrl
             * @memberof zed.scene.AtlasBounds
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            AtlasBounds.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.AtlasBounds";
            };

            return AtlasBounds;
        })();

        scene.AtlasTile = (function() {

            /**
             * Properties of an AtlasTile.
             * @memberof zed.scene
             * @interface IAtlasTile
             * @property {zed.scene.IAtlasTextureId|null} [textureId] AtlasTile textureId
             * @property {number|null} [tileId] AtlasTile tileId
             * @property {number|null} [padding] AtlasTile padding
             * @property {zed.scene.IAtlasBounds|null} [bounds] AtlasTile bounds
             */

            /**
             * Constructs a new AtlasTile.
             * @memberof zed.scene
             * @classdesc Represents an AtlasTile.
             * @implements IAtlasTile
             * @constructor
             * @param {zed.scene.IAtlasTile=} [properties] Properties to set
             */
            function AtlasTile(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * AtlasTile textureId.
             * @member {zed.scene.IAtlasTextureId|null|undefined} textureId
             * @memberof zed.scene.AtlasTile
             * @instance
             */
            AtlasTile.prototype.textureId = null;

            /**
             * AtlasTile tileId.
             * @member {number} tileId
             * @memberof zed.scene.AtlasTile
             * @instance
             */
            AtlasTile.prototype.tileId = 0;

            /**
             * AtlasTile padding.
             * @member {number} padding
             * @memberof zed.scene.AtlasTile
             * @instance
             */
            AtlasTile.prototype.padding = 0;

            /**
             * AtlasTile bounds.
             * @member {zed.scene.IAtlasBounds|null|undefined} bounds
             * @memberof zed.scene.AtlasTile
             * @instance
             */
            AtlasTile.prototype.bounds = null;

            /**
             * Creates a new AtlasTile instance using the specified properties.
             * @function create
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {zed.scene.IAtlasTile=} [properties] Properties to set
             * @returns {zed.scene.AtlasTile} AtlasTile instance
             */
            AtlasTile.create = function create(properties) {
                return new AtlasTile(properties);
            };

            /**
             * Encodes the specified AtlasTile message. Does not implicitly {@link zed.scene.AtlasTile.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {zed.scene.IAtlasTile} message AtlasTile message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasTile.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.textureId != null && Object.hasOwnProperty.call(message, "textureId"))
                    $root.zed.scene.AtlasTextureId.encode(message.textureId, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.tileId != null && Object.hasOwnProperty.call(message, "tileId"))
                    writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.tileId);
                if (message.padding != null && Object.hasOwnProperty.call(message, "padding"))
                    writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.padding);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.AtlasBounds.encode(message.bounds, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified AtlasTile message, length delimited. Does not implicitly {@link zed.scene.AtlasTile.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {zed.scene.IAtlasTile} message AtlasTile message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AtlasTile.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an AtlasTile message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.AtlasTile} AtlasTile
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasTile.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.AtlasTile();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.textureId = $root.zed.scene.AtlasTextureId.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.tileId = reader.uint32();
                            break;
                        }
                    case 3: {
                            message.padding = reader.uint32();
                            break;
                        }
                    case 4: {
                            message.bounds = $root.zed.scene.AtlasBounds.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an AtlasTile message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.AtlasTile} AtlasTile
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AtlasTile.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an AtlasTile message.
             * @function verify
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            AtlasTile.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.textureId != null && message.hasOwnProperty("textureId")) {
                    let error = $root.zed.scene.AtlasTextureId.verify(message.textureId);
                    if (error)
                        return "textureId." + error;
                }
                if (message.tileId != null && message.hasOwnProperty("tileId"))
                    if (!$util.isInteger(message.tileId))
                        return "tileId: integer expected";
                if (message.padding != null && message.hasOwnProperty("padding"))
                    if (!$util.isInteger(message.padding))
                        return "padding: integer expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.AtlasBounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                return null;
            };

            /**
             * Creates an AtlasTile message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.AtlasTile} AtlasTile
             */
            AtlasTile.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.AtlasTile)
                    return object;
                let message = new $root.zed.scene.AtlasTile();
                if (object.textureId != null) {
                    if (typeof object.textureId !== "object")
                        throw TypeError(".zed.scene.AtlasTile.textureId: object expected");
                    message.textureId = $root.zed.scene.AtlasTextureId.fromObject(object.textureId);
                }
                if (object.tileId != null)
                    message.tileId = object.tileId >>> 0;
                if (object.padding != null)
                    message.padding = object.padding >>> 0;
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.AtlasTile.bounds: object expected");
                    message.bounds = $root.zed.scene.AtlasBounds.fromObject(object.bounds);
                }
                return message;
            };

            /**
             * Creates a plain object from an AtlasTile message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {zed.scene.AtlasTile} message AtlasTile
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            AtlasTile.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.textureId = null;
                    object.tileId = 0;
                    object.padding = 0;
                    object.bounds = null;
                }
                if (message.textureId != null && message.hasOwnProperty("textureId"))
                    object.textureId = $root.zed.scene.AtlasTextureId.toObject(message.textureId, options);
                if (message.tileId != null && message.hasOwnProperty("tileId"))
                    object.tileId = message.tileId;
                if (message.padding != null && message.hasOwnProperty("padding"))
                    object.padding = message.padding;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.AtlasBounds.toObject(message.bounds, options);
                return object;
            };

            /**
             * Converts this AtlasTile to JSON.
             * @function toJSON
             * @memberof zed.scene.AtlasTile
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            AtlasTile.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for AtlasTile
             * @function getTypeUrl
             * @memberof zed.scene.AtlasTile
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            AtlasTile.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.AtlasTile";
            };

            return AtlasTile;
        })();

        scene.TransformationMatrix = (function() {

            /**
             * Properties of a TransformationMatrix.
             * @memberof zed.scene
             * @interface ITransformationMatrix
             * @property {number|null} [r00] TransformationMatrix r00
             * @property {number|null} [r01] TransformationMatrix r01
             * @property {number|null} [r10] TransformationMatrix r10
             * @property {number|null} [r11] TransformationMatrix r11
             * @property {number|null} [tx] TransformationMatrix tx
             * @property {number|null} [ty] TransformationMatrix ty
             */

            /**
             * Constructs a new TransformationMatrix.
             * @memberof zed.scene
             * @classdesc Represents a TransformationMatrix.
             * @implements ITransformationMatrix
             * @constructor
             * @param {zed.scene.ITransformationMatrix=} [properties] Properties to set
             */
            function TransformationMatrix(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * TransformationMatrix r00.
             * @member {number} r00
             * @memberof zed.scene.TransformationMatrix
             * @instance
             */
            TransformationMatrix.prototype.r00 = 0;

            /**
             * TransformationMatrix r01.
             * @member {number} r01
             * @memberof zed.scene.TransformationMatrix
             * @instance
             */
            TransformationMatrix.prototype.r01 = 0;

            /**
             * TransformationMatrix r10.
             * @member {number} r10
             * @memberof zed.scene.TransformationMatrix
             * @instance
             */
            TransformationMatrix.prototype.r10 = 0;

            /**
             * TransformationMatrix r11.
             * @member {number} r11
             * @memberof zed.scene.TransformationMatrix
             * @instance
             */
            TransformationMatrix.prototype.r11 = 0;

            /**
             * TransformationMatrix tx.
             * @member {number} tx
             * @memberof zed.scene.TransformationMatrix
             * @instance
             */
            TransformationMatrix.prototype.tx = 0;

            /**
             * TransformationMatrix ty.
             * @member {number} ty
             * @memberof zed.scene.TransformationMatrix
             * @instance
             */
            TransformationMatrix.prototype.ty = 0;

            /**
             * Creates a new TransformationMatrix instance using the specified properties.
             * @function create
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {zed.scene.ITransformationMatrix=} [properties] Properties to set
             * @returns {zed.scene.TransformationMatrix} TransformationMatrix instance
             */
            TransformationMatrix.create = function create(properties) {
                return new TransformationMatrix(properties);
            };

            /**
             * Encodes the specified TransformationMatrix message. Does not implicitly {@link zed.scene.TransformationMatrix.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {zed.scene.ITransformationMatrix} message TransformationMatrix message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            TransformationMatrix.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.r00 != null && Object.hasOwnProperty.call(message, "r00"))
                    writer.uint32(/* id 1, wireType 5 =*/13).float(message.r00);
                if (message.r01 != null && Object.hasOwnProperty.call(message, "r01"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.r01);
                if (message.r10 != null && Object.hasOwnProperty.call(message, "r10"))
                    writer.uint32(/* id 3, wireType 5 =*/29).float(message.r10);
                if (message.r11 != null && Object.hasOwnProperty.call(message, "r11"))
                    writer.uint32(/* id 4, wireType 5 =*/37).float(message.r11);
                if (message.tx != null && Object.hasOwnProperty.call(message, "tx"))
                    writer.uint32(/* id 5, wireType 5 =*/45).float(message.tx);
                if (message.ty != null && Object.hasOwnProperty.call(message, "ty"))
                    writer.uint32(/* id 6, wireType 5 =*/53).float(message.ty);
                return writer;
            };

            /**
             * Encodes the specified TransformationMatrix message, length delimited. Does not implicitly {@link zed.scene.TransformationMatrix.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {zed.scene.ITransformationMatrix} message TransformationMatrix message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            TransformationMatrix.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a TransformationMatrix message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.TransformationMatrix} TransformationMatrix
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            TransformationMatrix.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.TransformationMatrix();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.r00 = reader.float();
                            break;
                        }
                    case 2: {
                            message.r01 = reader.float();
                            break;
                        }
                    case 3: {
                            message.r10 = reader.float();
                            break;
                        }
                    case 4: {
                            message.r11 = reader.float();
                            break;
                        }
                    case 5: {
                            message.tx = reader.float();
                            break;
                        }
                    case 6: {
                            message.ty = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a TransformationMatrix message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.TransformationMatrix} TransformationMatrix
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            TransformationMatrix.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a TransformationMatrix message.
             * @function verify
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            TransformationMatrix.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.r00 != null && message.hasOwnProperty("r00"))
                    if (typeof message.r00 !== "number")
                        return "r00: number expected";
                if (message.r01 != null && message.hasOwnProperty("r01"))
                    if (typeof message.r01 !== "number")
                        return "r01: number expected";
                if (message.r10 != null && message.hasOwnProperty("r10"))
                    if (typeof message.r10 !== "number")
                        return "r10: number expected";
                if (message.r11 != null && message.hasOwnProperty("r11"))
                    if (typeof message.r11 !== "number")
                        return "r11: number expected";
                if (message.tx != null && message.hasOwnProperty("tx"))
                    if (typeof message.tx !== "number")
                        return "tx: number expected";
                if (message.ty != null && message.hasOwnProperty("ty"))
                    if (typeof message.ty !== "number")
                        return "ty: number expected";
                return null;
            };

            /**
             * Creates a TransformationMatrix message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.TransformationMatrix} TransformationMatrix
             */
            TransformationMatrix.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.TransformationMatrix)
                    return object;
                let message = new $root.zed.scene.TransformationMatrix();
                if (object.r00 != null)
                    message.r00 = Number(object.r00);
                if (object.r01 != null)
                    message.r01 = Number(object.r01);
                if (object.r10 != null)
                    message.r10 = Number(object.r10);
                if (object.r11 != null)
                    message.r11 = Number(object.r11);
                if (object.tx != null)
                    message.tx = Number(object.tx);
                if (object.ty != null)
                    message.ty = Number(object.ty);
                return message;
            };

            /**
             * Creates a plain object from a TransformationMatrix message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {zed.scene.TransformationMatrix} message TransformationMatrix
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            TransformationMatrix.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.r00 = 0;
                    object.r01 = 0;
                    object.r10 = 0;
                    object.r11 = 0;
                    object.tx = 0;
                    object.ty = 0;
                }
                if (message.r00 != null && message.hasOwnProperty("r00"))
                    object.r00 = options.json && !isFinite(message.r00) ? String(message.r00) : message.r00;
                if (message.r01 != null && message.hasOwnProperty("r01"))
                    object.r01 = options.json && !isFinite(message.r01) ? String(message.r01) : message.r01;
                if (message.r10 != null && message.hasOwnProperty("r10"))
                    object.r10 = options.json && !isFinite(message.r10) ? String(message.r10) : message.r10;
                if (message.r11 != null && message.hasOwnProperty("r11"))
                    object.r11 = options.json && !isFinite(message.r11) ? String(message.r11) : message.r11;
                if (message.tx != null && message.hasOwnProperty("tx"))
                    object.tx = options.json && !isFinite(message.tx) ? String(message.tx) : message.tx;
                if (message.ty != null && message.hasOwnProperty("ty"))
                    object.ty = options.json && !isFinite(message.ty) ? String(message.ty) : message.ty;
                return object;
            };

            /**
             * Converts this TransformationMatrix to JSON.
             * @function toJSON
             * @memberof zed.scene.TransformationMatrix
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            TransformationMatrix.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for TransformationMatrix
             * @function getTypeUrl
             * @memberof zed.scene.TransformationMatrix
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            TransformationMatrix.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.TransformationMatrix";
            };

            return TransformationMatrix;
        })();

        scene.Shadow = (function() {

            /**
             * Properties of a Shadow.
             * @memberof zed.scene
             * @interface IShadow
             * @property {number|null} [order] Shadow order
             * @property {number|null} [blurRadius] Shadow blurRadius
             * @property {zed.scene.IBounds|null} [bounds] Shadow bounds
             * @property {zed.scene.ICorners|null} [cornerRadii] Shadow cornerRadii
             * @property {zed.scene.IContentMask|null} [contentMask] Shadow contentMask
             * @property {zed.scene.IHsla|null} [color] Shadow color
             */

            /**
             * Constructs a new Shadow.
             * @memberof zed.scene
             * @classdesc Represents a Shadow.
             * @implements IShadow
             * @constructor
             * @param {zed.scene.IShadow=} [properties] Properties to set
             */
            function Shadow(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Shadow order.
             * @member {number} order
             * @memberof zed.scene.Shadow
             * @instance
             */
            Shadow.prototype.order = 0;

            /**
             * Shadow blurRadius.
             * @member {number} blurRadius
             * @memberof zed.scene.Shadow
             * @instance
             */
            Shadow.prototype.blurRadius = 0;

            /**
             * Shadow bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.Shadow
             * @instance
             */
            Shadow.prototype.bounds = null;

            /**
             * Shadow cornerRadii.
             * @member {zed.scene.ICorners|null|undefined} cornerRadii
             * @memberof zed.scene.Shadow
             * @instance
             */
            Shadow.prototype.cornerRadii = null;

            /**
             * Shadow contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.Shadow
             * @instance
             */
            Shadow.prototype.contentMask = null;

            /**
             * Shadow color.
             * @member {zed.scene.IHsla|null|undefined} color
             * @memberof zed.scene.Shadow
             * @instance
             */
            Shadow.prototype.color = null;

            /**
             * Creates a new Shadow instance using the specified properties.
             * @function create
             * @memberof zed.scene.Shadow
             * @static
             * @param {zed.scene.IShadow=} [properties] Properties to set
             * @returns {zed.scene.Shadow} Shadow instance
             */
            Shadow.create = function create(properties) {
                return new Shadow(properties);
            };

            /**
             * Encodes the specified Shadow message. Does not implicitly {@link zed.scene.Shadow.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Shadow
             * @static
             * @param {zed.scene.IShadow} message Shadow message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Shadow.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.blurRadius != null && Object.hasOwnProperty.call(message, "blurRadius"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.blurRadius);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.cornerRadii != null && Object.hasOwnProperty.call(message, "cornerRadii"))
                    $root.zed.scene.Corners.encode(message.cornerRadii, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.color != null && Object.hasOwnProperty.call(message, "color"))
                    $root.zed.scene.Hsla.encode(message.color, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified Shadow message, length delimited. Does not implicitly {@link zed.scene.Shadow.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Shadow
             * @static
             * @param {zed.scene.IShadow} message Shadow message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Shadow.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Shadow message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Shadow
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Shadow} Shadow
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Shadow.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Shadow();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.blurRadius = reader.float();
                            break;
                        }
                    case 3: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.cornerRadii = $root.zed.scene.Corners.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 6: {
                            message.color = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Shadow message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Shadow
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Shadow} Shadow
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Shadow.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Shadow message.
             * @function verify
             * @memberof zed.scene.Shadow
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Shadow.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.blurRadius != null && message.hasOwnProperty("blurRadius"))
                    if (typeof message.blurRadius !== "number")
                        return "blurRadius: number expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.cornerRadii != null && message.hasOwnProperty("cornerRadii")) {
                    let error = $root.zed.scene.Corners.verify(message.cornerRadii);
                    if (error)
                        return "cornerRadii." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.color != null && message.hasOwnProperty("color")) {
                    let error = $root.zed.scene.Hsla.verify(message.color);
                    if (error)
                        return "color." + error;
                }
                return null;
            };

            /**
             * Creates a Shadow message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Shadow
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Shadow} Shadow
             */
            Shadow.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Shadow)
                    return object;
                let message = new $root.zed.scene.Shadow();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.blurRadius != null)
                    message.blurRadius = Number(object.blurRadius);
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.Shadow.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.cornerRadii != null) {
                    if (typeof object.cornerRadii !== "object")
                        throw TypeError(".zed.scene.Shadow.cornerRadii: object expected");
                    message.cornerRadii = $root.zed.scene.Corners.fromObject(object.cornerRadii);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.Shadow.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.color != null) {
                    if (typeof object.color !== "object")
                        throw TypeError(".zed.scene.Shadow.color: object expected");
                    message.color = $root.zed.scene.Hsla.fromObject(object.color);
                }
                return message;
            };

            /**
             * Creates a plain object from a Shadow message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Shadow
             * @static
             * @param {zed.scene.Shadow} message Shadow
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Shadow.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.order = 0;
                    object.blurRadius = 0;
                    object.bounds = null;
                    object.cornerRadii = null;
                    object.contentMask = null;
                    object.color = null;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.blurRadius != null && message.hasOwnProperty("blurRadius"))
                    object.blurRadius = options.json && !isFinite(message.blurRadius) ? String(message.blurRadius) : message.blurRadius;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.cornerRadii != null && message.hasOwnProperty("cornerRadii"))
                    object.cornerRadii = $root.zed.scene.Corners.toObject(message.cornerRadii, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.color != null && message.hasOwnProperty("color"))
                    object.color = $root.zed.scene.Hsla.toObject(message.color, options);
                return object;
            };

            /**
             * Converts this Shadow to JSON.
             * @function toJSON
             * @memberof zed.scene.Shadow
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Shadow.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Shadow
             * @function getTypeUrl
             * @memberof zed.scene.Shadow
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Shadow.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Shadow";
            };

            return Shadow;
        })();

        scene.Quad = (function() {

            /**
             * Properties of a Quad.
             * @memberof zed.scene
             * @interface IQuad
             * @property {number|null} [order] Quad order
             * @property {number|null} [borderStyle] Quad borderStyle
             * @property {zed.scene.IBounds|null} [bounds] Quad bounds
             * @property {zed.scene.IContentMask|null} [contentMask] Quad contentMask
             * @property {zed.scene.IBackground|null} [background] Quad background
             * @property {zed.scene.IHsla|null} [borderColor] Quad borderColor
             * @property {zed.scene.ICorners|null} [cornerRadii] Quad cornerRadii
             * @property {zed.scene.IEdges|null} [borderWidths] Quad borderWidths
             */

            /**
             * Constructs a new Quad.
             * @memberof zed.scene
             * @classdesc Represents a Quad.
             * @implements IQuad
             * @constructor
             * @param {zed.scene.IQuad=} [properties] Properties to set
             */
            function Quad(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Quad order.
             * @member {number} order
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.order = 0;

            /**
             * Quad borderStyle.
             * @member {number} borderStyle
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.borderStyle = 0;

            /**
             * Quad bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.bounds = null;

            /**
             * Quad contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.contentMask = null;

            /**
             * Quad background.
             * @member {zed.scene.IBackground|null|undefined} background
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.background = null;

            /**
             * Quad borderColor.
             * @member {zed.scene.IHsla|null|undefined} borderColor
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.borderColor = null;

            /**
             * Quad cornerRadii.
             * @member {zed.scene.ICorners|null|undefined} cornerRadii
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.cornerRadii = null;

            /**
             * Quad borderWidths.
             * @member {zed.scene.IEdges|null|undefined} borderWidths
             * @memberof zed.scene.Quad
             * @instance
             */
            Quad.prototype.borderWidths = null;

            /**
             * Creates a new Quad instance using the specified properties.
             * @function create
             * @memberof zed.scene.Quad
             * @static
             * @param {zed.scene.IQuad=} [properties] Properties to set
             * @returns {zed.scene.Quad} Quad instance
             */
            Quad.create = function create(properties) {
                return new Quad(properties);
            };

            /**
             * Encodes the specified Quad message. Does not implicitly {@link zed.scene.Quad.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Quad
             * @static
             * @param {zed.scene.IQuad} message Quad message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Quad.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.borderStyle != null && Object.hasOwnProperty.call(message, "borderStyle"))
                    writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.borderStyle);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.background != null && Object.hasOwnProperty.call(message, "background"))
                    $root.zed.scene.Background.encode(message.background, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.borderColor != null && Object.hasOwnProperty.call(message, "borderColor"))
                    $root.zed.scene.Hsla.encode(message.borderColor, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                if (message.cornerRadii != null && Object.hasOwnProperty.call(message, "cornerRadii"))
                    $root.zed.scene.Corners.encode(message.cornerRadii, writer.uint32(/* id 7, wireType 2 =*/58).fork()).ldelim();
                if (message.borderWidths != null && Object.hasOwnProperty.call(message, "borderWidths"))
                    $root.zed.scene.Edges.encode(message.borderWidths, writer.uint32(/* id 8, wireType 2 =*/66).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified Quad message, length delimited. Does not implicitly {@link zed.scene.Quad.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Quad
             * @static
             * @param {zed.scene.IQuad} message Quad message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Quad.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Quad message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Quad
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Quad} Quad
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Quad.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Quad();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.borderStyle = reader.uint32();
                            break;
                        }
                    case 3: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.background = $root.zed.scene.Background.decode(reader, reader.uint32());
                            break;
                        }
                    case 6: {
                            message.borderColor = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 7: {
                            message.cornerRadii = $root.zed.scene.Corners.decode(reader, reader.uint32());
                            break;
                        }
                    case 8: {
                            message.borderWidths = $root.zed.scene.Edges.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Quad message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Quad
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Quad} Quad
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Quad.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Quad message.
             * @function verify
             * @memberof zed.scene.Quad
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Quad.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.borderStyle != null && message.hasOwnProperty("borderStyle"))
                    if (!$util.isInteger(message.borderStyle))
                        return "borderStyle: integer expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.background != null && message.hasOwnProperty("background")) {
                    let error = $root.zed.scene.Background.verify(message.background);
                    if (error)
                        return "background." + error;
                }
                if (message.borderColor != null && message.hasOwnProperty("borderColor")) {
                    let error = $root.zed.scene.Hsla.verify(message.borderColor);
                    if (error)
                        return "borderColor." + error;
                }
                if (message.cornerRadii != null && message.hasOwnProperty("cornerRadii")) {
                    let error = $root.zed.scene.Corners.verify(message.cornerRadii);
                    if (error)
                        return "cornerRadii." + error;
                }
                if (message.borderWidths != null && message.hasOwnProperty("borderWidths")) {
                    let error = $root.zed.scene.Edges.verify(message.borderWidths);
                    if (error)
                        return "borderWidths." + error;
                }
                return null;
            };

            /**
             * Creates a Quad message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Quad
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Quad} Quad
             */
            Quad.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Quad)
                    return object;
                let message = new $root.zed.scene.Quad();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.borderStyle != null)
                    message.borderStyle = object.borderStyle >>> 0;
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.Quad.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.Quad.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.background != null) {
                    if (typeof object.background !== "object")
                        throw TypeError(".zed.scene.Quad.background: object expected");
                    message.background = $root.zed.scene.Background.fromObject(object.background);
                }
                if (object.borderColor != null) {
                    if (typeof object.borderColor !== "object")
                        throw TypeError(".zed.scene.Quad.borderColor: object expected");
                    message.borderColor = $root.zed.scene.Hsla.fromObject(object.borderColor);
                }
                if (object.cornerRadii != null) {
                    if (typeof object.cornerRadii !== "object")
                        throw TypeError(".zed.scene.Quad.cornerRadii: object expected");
                    message.cornerRadii = $root.zed.scene.Corners.fromObject(object.cornerRadii);
                }
                if (object.borderWidths != null) {
                    if (typeof object.borderWidths !== "object")
                        throw TypeError(".zed.scene.Quad.borderWidths: object expected");
                    message.borderWidths = $root.zed.scene.Edges.fromObject(object.borderWidths);
                }
                return message;
            };

            /**
             * Creates a plain object from a Quad message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Quad
             * @static
             * @param {zed.scene.Quad} message Quad
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Quad.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.order = 0;
                    object.borderStyle = 0;
                    object.bounds = null;
                    object.contentMask = null;
                    object.background = null;
                    object.borderColor = null;
                    object.cornerRadii = null;
                    object.borderWidths = null;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.borderStyle != null && message.hasOwnProperty("borderStyle"))
                    object.borderStyle = message.borderStyle;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.background != null && message.hasOwnProperty("background"))
                    object.background = $root.zed.scene.Background.toObject(message.background, options);
                if (message.borderColor != null && message.hasOwnProperty("borderColor"))
                    object.borderColor = $root.zed.scene.Hsla.toObject(message.borderColor, options);
                if (message.cornerRadii != null && message.hasOwnProperty("cornerRadii"))
                    object.cornerRadii = $root.zed.scene.Corners.toObject(message.cornerRadii, options);
                if (message.borderWidths != null && message.hasOwnProperty("borderWidths"))
                    object.borderWidths = $root.zed.scene.Edges.toObject(message.borderWidths, options);
                return object;
            };

            /**
             * Converts this Quad to JSON.
             * @function toJSON
             * @memberof zed.scene.Quad
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Quad.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Quad
             * @function getTypeUrl
             * @memberof zed.scene.Quad
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Quad.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Quad";
            };

            return Quad;
        })();

        scene.Underline = (function() {

            /**
             * Properties of an Underline.
             * @memberof zed.scene
             * @interface IUnderline
             * @property {number|null} [order] Underline order
             * @property {zed.scene.IBounds|null} [bounds] Underline bounds
             * @property {zed.scene.IContentMask|null} [contentMask] Underline contentMask
             * @property {zed.scene.IHsla|null} [color] Underline color
             * @property {number|null} [thickness] Underline thickness
             * @property {boolean|null} [wavy] Underline wavy
             */

            /**
             * Constructs a new Underline.
             * @memberof zed.scene
             * @classdesc Represents an Underline.
             * @implements IUnderline
             * @constructor
             * @param {zed.scene.IUnderline=} [properties] Properties to set
             */
            function Underline(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Underline order.
             * @member {number} order
             * @memberof zed.scene.Underline
             * @instance
             */
            Underline.prototype.order = 0;

            /**
             * Underline bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.Underline
             * @instance
             */
            Underline.prototype.bounds = null;

            /**
             * Underline contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.Underline
             * @instance
             */
            Underline.prototype.contentMask = null;

            /**
             * Underline color.
             * @member {zed.scene.IHsla|null|undefined} color
             * @memberof zed.scene.Underline
             * @instance
             */
            Underline.prototype.color = null;

            /**
             * Underline thickness.
             * @member {number} thickness
             * @memberof zed.scene.Underline
             * @instance
             */
            Underline.prototype.thickness = 0;

            /**
             * Underline wavy.
             * @member {boolean} wavy
             * @memberof zed.scene.Underline
             * @instance
             */
            Underline.prototype.wavy = false;

            /**
             * Creates a new Underline instance using the specified properties.
             * @function create
             * @memberof zed.scene.Underline
             * @static
             * @param {zed.scene.IUnderline=} [properties] Properties to set
             * @returns {zed.scene.Underline} Underline instance
             */
            Underline.create = function create(properties) {
                return new Underline(properties);
            };

            /**
             * Encodes the specified Underline message. Does not implicitly {@link zed.scene.Underline.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Underline
             * @static
             * @param {zed.scene.IUnderline} message Underline message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Underline.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.color != null && Object.hasOwnProperty.call(message, "color"))
                    $root.zed.scene.Hsla.encode(message.color, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.thickness != null && Object.hasOwnProperty.call(message, "thickness"))
                    writer.uint32(/* id 5, wireType 5 =*/45).float(message.thickness);
                if (message.wavy != null && Object.hasOwnProperty.call(message, "wavy"))
                    writer.uint32(/* id 6, wireType 0 =*/48).bool(message.wavy);
                return writer;
            };

            /**
             * Encodes the specified Underline message, length delimited. Does not implicitly {@link zed.scene.Underline.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Underline
             * @static
             * @param {zed.scene.IUnderline} message Underline message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Underline.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an Underline message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Underline
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Underline} Underline
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Underline.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Underline();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.color = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.thickness = reader.float();
                            break;
                        }
                    case 6: {
                            message.wavy = reader.bool();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an Underline message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Underline
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Underline} Underline
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Underline.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an Underline message.
             * @function verify
             * @memberof zed.scene.Underline
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Underline.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.color != null && message.hasOwnProperty("color")) {
                    let error = $root.zed.scene.Hsla.verify(message.color);
                    if (error)
                        return "color." + error;
                }
                if (message.thickness != null && message.hasOwnProperty("thickness"))
                    if (typeof message.thickness !== "number")
                        return "thickness: number expected";
                if (message.wavy != null && message.hasOwnProperty("wavy"))
                    if (typeof message.wavy !== "boolean")
                        return "wavy: boolean expected";
                return null;
            };

            /**
             * Creates an Underline message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Underline
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Underline} Underline
             */
            Underline.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Underline)
                    return object;
                let message = new $root.zed.scene.Underline();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.Underline.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.Underline.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.color != null) {
                    if (typeof object.color !== "object")
                        throw TypeError(".zed.scene.Underline.color: object expected");
                    message.color = $root.zed.scene.Hsla.fromObject(object.color);
                }
                if (object.thickness != null)
                    message.thickness = Number(object.thickness);
                if (object.wavy != null)
                    message.wavy = Boolean(object.wavy);
                return message;
            };

            /**
             * Creates a plain object from an Underline message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Underline
             * @static
             * @param {zed.scene.Underline} message Underline
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Underline.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.order = 0;
                    object.bounds = null;
                    object.contentMask = null;
                    object.color = null;
                    object.thickness = 0;
                    object.wavy = false;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.color != null && message.hasOwnProperty("color"))
                    object.color = $root.zed.scene.Hsla.toObject(message.color, options);
                if (message.thickness != null && message.hasOwnProperty("thickness"))
                    object.thickness = options.json && !isFinite(message.thickness) ? String(message.thickness) : message.thickness;
                if (message.wavy != null && message.hasOwnProperty("wavy"))
                    object.wavy = message.wavy;
                return object;
            };

            /**
             * Converts this Underline to JSON.
             * @function toJSON
             * @memberof zed.scene.Underline
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Underline.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Underline
             * @function getTypeUrl
             * @memberof zed.scene.Underline
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Underline.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Underline";
            };

            return Underline;
        })();

        scene.MonochromeSprite = (function() {

            /**
             * Properties of a MonochromeSprite.
             * @memberof zed.scene
             * @interface IMonochromeSprite
             * @property {number|null} [order] MonochromeSprite order
             * @property {zed.scene.IBounds|null} [bounds] MonochromeSprite bounds
             * @property {zed.scene.IContentMask|null} [contentMask] MonochromeSprite contentMask
             * @property {zed.scene.IHsla|null} [color] MonochromeSprite color
             * @property {zed.scene.IAtlasTile|null} [tile] MonochromeSprite tile
             * @property {zed.scene.ITransformationMatrix|null} [transformation] MonochromeSprite transformation
             */

            /**
             * Constructs a new MonochromeSprite.
             * @memberof zed.scene
             * @classdesc Represents a MonochromeSprite.
             * @implements IMonochromeSprite
             * @constructor
             * @param {zed.scene.IMonochromeSprite=} [properties] Properties to set
             */
            function MonochromeSprite(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * MonochromeSprite order.
             * @member {number} order
             * @memberof zed.scene.MonochromeSprite
             * @instance
             */
            MonochromeSprite.prototype.order = 0;

            /**
             * MonochromeSprite bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.MonochromeSprite
             * @instance
             */
            MonochromeSprite.prototype.bounds = null;

            /**
             * MonochromeSprite contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.MonochromeSprite
             * @instance
             */
            MonochromeSprite.prototype.contentMask = null;

            /**
             * MonochromeSprite color.
             * @member {zed.scene.IHsla|null|undefined} color
             * @memberof zed.scene.MonochromeSprite
             * @instance
             */
            MonochromeSprite.prototype.color = null;

            /**
             * MonochromeSprite tile.
             * @member {zed.scene.IAtlasTile|null|undefined} tile
             * @memberof zed.scene.MonochromeSprite
             * @instance
             */
            MonochromeSprite.prototype.tile = null;

            /**
             * MonochromeSprite transformation.
             * @member {zed.scene.ITransformationMatrix|null|undefined} transformation
             * @memberof zed.scene.MonochromeSprite
             * @instance
             */
            MonochromeSprite.prototype.transformation = null;

            /**
             * Creates a new MonochromeSprite instance using the specified properties.
             * @function create
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {zed.scene.IMonochromeSprite=} [properties] Properties to set
             * @returns {zed.scene.MonochromeSprite} MonochromeSprite instance
             */
            MonochromeSprite.create = function create(properties) {
                return new MonochromeSprite(properties);
            };

            /**
             * Encodes the specified MonochromeSprite message. Does not implicitly {@link zed.scene.MonochromeSprite.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {zed.scene.IMonochromeSprite} message MonochromeSprite message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MonochromeSprite.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.color != null && Object.hasOwnProperty.call(message, "color"))
                    $root.zed.scene.Hsla.encode(message.color, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.tile != null && Object.hasOwnProperty.call(message, "tile"))
                    $root.zed.scene.AtlasTile.encode(message.tile, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.transformation != null && Object.hasOwnProperty.call(message, "transformation"))
                    $root.zed.scene.TransformationMatrix.encode(message.transformation, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified MonochromeSprite message, length delimited. Does not implicitly {@link zed.scene.MonochromeSprite.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {zed.scene.IMonochromeSprite} message MonochromeSprite message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MonochromeSprite.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a MonochromeSprite message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.MonochromeSprite} MonochromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MonochromeSprite.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.MonochromeSprite();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.color = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.tile = $root.zed.scene.AtlasTile.decode(reader, reader.uint32());
                            break;
                        }
                    case 6: {
                            message.transformation = $root.zed.scene.TransformationMatrix.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a MonochromeSprite message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.MonochromeSprite} MonochromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MonochromeSprite.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a MonochromeSprite message.
             * @function verify
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            MonochromeSprite.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.color != null && message.hasOwnProperty("color")) {
                    let error = $root.zed.scene.Hsla.verify(message.color);
                    if (error)
                        return "color." + error;
                }
                if (message.tile != null && message.hasOwnProperty("tile")) {
                    let error = $root.zed.scene.AtlasTile.verify(message.tile);
                    if (error)
                        return "tile." + error;
                }
                if (message.transformation != null && message.hasOwnProperty("transformation")) {
                    let error = $root.zed.scene.TransformationMatrix.verify(message.transformation);
                    if (error)
                        return "transformation." + error;
                }
                return null;
            };

            /**
             * Creates a MonochromeSprite message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.MonochromeSprite} MonochromeSprite
             */
            MonochromeSprite.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.MonochromeSprite)
                    return object;
                let message = new $root.zed.scene.MonochromeSprite();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.MonochromeSprite.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.MonochromeSprite.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.color != null) {
                    if (typeof object.color !== "object")
                        throw TypeError(".zed.scene.MonochromeSprite.color: object expected");
                    message.color = $root.zed.scene.Hsla.fromObject(object.color);
                }
                if (object.tile != null) {
                    if (typeof object.tile !== "object")
                        throw TypeError(".zed.scene.MonochromeSprite.tile: object expected");
                    message.tile = $root.zed.scene.AtlasTile.fromObject(object.tile);
                }
                if (object.transformation != null) {
                    if (typeof object.transformation !== "object")
                        throw TypeError(".zed.scene.MonochromeSprite.transformation: object expected");
                    message.transformation = $root.zed.scene.TransformationMatrix.fromObject(object.transformation);
                }
                return message;
            };

            /**
             * Creates a plain object from a MonochromeSprite message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {zed.scene.MonochromeSprite} message MonochromeSprite
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            MonochromeSprite.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.order = 0;
                    object.bounds = null;
                    object.contentMask = null;
                    object.color = null;
                    object.tile = null;
                    object.transformation = null;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.color != null && message.hasOwnProperty("color"))
                    object.color = $root.zed.scene.Hsla.toObject(message.color, options);
                if (message.tile != null && message.hasOwnProperty("tile"))
                    object.tile = $root.zed.scene.AtlasTile.toObject(message.tile, options);
                if (message.transformation != null && message.hasOwnProperty("transformation"))
                    object.transformation = $root.zed.scene.TransformationMatrix.toObject(message.transformation, options);
                return object;
            };

            /**
             * Converts this MonochromeSprite to JSON.
             * @function toJSON
             * @memberof zed.scene.MonochromeSprite
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            MonochromeSprite.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for MonochromeSprite
             * @function getTypeUrl
             * @memberof zed.scene.MonochromeSprite
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            MonochromeSprite.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.MonochromeSprite";
            };

            return MonochromeSprite;
        })();

        scene.SubpixelSprite = (function() {

            /**
             * Properties of a SubpixelSprite.
             * @memberof zed.scene
             * @interface ISubpixelSprite
             * @property {number|null} [order] SubpixelSprite order
             * @property {zed.scene.IBounds|null} [bounds] SubpixelSprite bounds
             * @property {zed.scene.IContentMask|null} [contentMask] SubpixelSprite contentMask
             * @property {zed.scene.IHsla|null} [color] SubpixelSprite color
             * @property {zed.scene.IAtlasTile|null} [tile] SubpixelSprite tile
             * @property {zed.scene.ITransformationMatrix|null} [transformation] SubpixelSprite transformation
             */

            /**
             * Constructs a new SubpixelSprite.
             * @memberof zed.scene
             * @classdesc Represents a SubpixelSprite.
             * @implements ISubpixelSprite
             * @constructor
             * @param {zed.scene.ISubpixelSprite=} [properties] Properties to set
             */
            function SubpixelSprite(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * SubpixelSprite order.
             * @member {number} order
             * @memberof zed.scene.SubpixelSprite
             * @instance
             */
            SubpixelSprite.prototype.order = 0;

            /**
             * SubpixelSprite bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.SubpixelSprite
             * @instance
             */
            SubpixelSprite.prototype.bounds = null;

            /**
             * SubpixelSprite contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.SubpixelSprite
             * @instance
             */
            SubpixelSprite.prototype.contentMask = null;

            /**
             * SubpixelSprite color.
             * @member {zed.scene.IHsla|null|undefined} color
             * @memberof zed.scene.SubpixelSprite
             * @instance
             */
            SubpixelSprite.prototype.color = null;

            /**
             * SubpixelSprite tile.
             * @member {zed.scene.IAtlasTile|null|undefined} tile
             * @memberof zed.scene.SubpixelSprite
             * @instance
             */
            SubpixelSprite.prototype.tile = null;

            /**
             * SubpixelSprite transformation.
             * @member {zed.scene.ITransformationMatrix|null|undefined} transformation
             * @memberof zed.scene.SubpixelSprite
             * @instance
             */
            SubpixelSprite.prototype.transformation = null;

            /**
             * Creates a new SubpixelSprite instance using the specified properties.
             * @function create
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {zed.scene.ISubpixelSprite=} [properties] Properties to set
             * @returns {zed.scene.SubpixelSprite} SubpixelSprite instance
             */
            SubpixelSprite.create = function create(properties) {
                return new SubpixelSprite(properties);
            };

            /**
             * Encodes the specified SubpixelSprite message. Does not implicitly {@link zed.scene.SubpixelSprite.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {zed.scene.ISubpixelSprite} message SubpixelSprite message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SubpixelSprite.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.color != null && Object.hasOwnProperty.call(message, "color"))
                    $root.zed.scene.Hsla.encode(message.color, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.tile != null && Object.hasOwnProperty.call(message, "tile"))
                    $root.zed.scene.AtlasTile.encode(message.tile, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.transformation != null && Object.hasOwnProperty.call(message, "transformation"))
                    $root.zed.scene.TransformationMatrix.encode(message.transformation, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified SubpixelSprite message, length delimited. Does not implicitly {@link zed.scene.SubpixelSprite.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {zed.scene.ISubpixelSprite} message SubpixelSprite message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SubpixelSprite.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a SubpixelSprite message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.SubpixelSprite} SubpixelSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SubpixelSprite.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.SubpixelSprite();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.color = $root.zed.scene.Hsla.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.tile = $root.zed.scene.AtlasTile.decode(reader, reader.uint32());
                            break;
                        }
                    case 6: {
                            message.transformation = $root.zed.scene.TransformationMatrix.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a SubpixelSprite message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.SubpixelSprite} SubpixelSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SubpixelSprite.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a SubpixelSprite message.
             * @function verify
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            SubpixelSprite.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.color != null && message.hasOwnProperty("color")) {
                    let error = $root.zed.scene.Hsla.verify(message.color);
                    if (error)
                        return "color." + error;
                }
                if (message.tile != null && message.hasOwnProperty("tile")) {
                    let error = $root.zed.scene.AtlasTile.verify(message.tile);
                    if (error)
                        return "tile." + error;
                }
                if (message.transformation != null && message.hasOwnProperty("transformation")) {
                    let error = $root.zed.scene.TransformationMatrix.verify(message.transformation);
                    if (error)
                        return "transformation." + error;
                }
                return null;
            };

            /**
             * Creates a SubpixelSprite message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.SubpixelSprite} SubpixelSprite
             */
            SubpixelSprite.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.SubpixelSprite)
                    return object;
                let message = new $root.zed.scene.SubpixelSprite();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.SubpixelSprite.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.SubpixelSprite.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.color != null) {
                    if (typeof object.color !== "object")
                        throw TypeError(".zed.scene.SubpixelSprite.color: object expected");
                    message.color = $root.zed.scene.Hsla.fromObject(object.color);
                }
                if (object.tile != null) {
                    if (typeof object.tile !== "object")
                        throw TypeError(".zed.scene.SubpixelSprite.tile: object expected");
                    message.tile = $root.zed.scene.AtlasTile.fromObject(object.tile);
                }
                if (object.transformation != null) {
                    if (typeof object.transformation !== "object")
                        throw TypeError(".zed.scene.SubpixelSprite.transformation: object expected");
                    message.transformation = $root.zed.scene.TransformationMatrix.fromObject(object.transformation);
                }
                return message;
            };

            /**
             * Creates a plain object from a SubpixelSprite message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {zed.scene.SubpixelSprite} message SubpixelSprite
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            SubpixelSprite.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.order = 0;
                    object.bounds = null;
                    object.contentMask = null;
                    object.color = null;
                    object.tile = null;
                    object.transformation = null;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.color != null && message.hasOwnProperty("color"))
                    object.color = $root.zed.scene.Hsla.toObject(message.color, options);
                if (message.tile != null && message.hasOwnProperty("tile"))
                    object.tile = $root.zed.scene.AtlasTile.toObject(message.tile, options);
                if (message.transformation != null && message.hasOwnProperty("transformation"))
                    object.transformation = $root.zed.scene.TransformationMatrix.toObject(message.transformation, options);
                return object;
            };

            /**
             * Converts this SubpixelSprite to JSON.
             * @function toJSON
             * @memberof zed.scene.SubpixelSprite
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            SubpixelSprite.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for SubpixelSprite
             * @function getTypeUrl
             * @memberof zed.scene.SubpixelSprite
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            SubpixelSprite.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.SubpixelSprite";
            };

            return SubpixelSprite;
        })();

        scene.PolychromeSprite = (function() {

            /**
             * Properties of a PolychromeSprite.
             * @memberof zed.scene
             * @interface IPolychromeSprite
             * @property {number|null} [order] PolychromeSprite order
             * @property {boolean|null} [grayscale] PolychromeSprite grayscale
             * @property {number|null} [opacity] PolychromeSprite opacity
             * @property {zed.scene.IBounds|null} [bounds] PolychromeSprite bounds
             * @property {zed.scene.IContentMask|null} [contentMask] PolychromeSprite contentMask
             * @property {zed.scene.ICorners|null} [cornerRadii] PolychromeSprite cornerRadii
             * @property {zed.scene.IAtlasTile|null} [tile] PolychromeSprite tile
             */

            /**
             * Constructs a new PolychromeSprite.
             * @memberof zed.scene
             * @classdesc Represents a PolychromeSprite.
             * @implements IPolychromeSprite
             * @constructor
             * @param {zed.scene.IPolychromeSprite=} [properties] Properties to set
             */
            function PolychromeSprite(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * PolychromeSprite order.
             * @member {number} order
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.order = 0;

            /**
             * PolychromeSprite grayscale.
             * @member {boolean} grayscale
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.grayscale = false;

            /**
             * PolychromeSprite opacity.
             * @member {number} opacity
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.opacity = 0;

            /**
             * PolychromeSprite bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.bounds = null;

            /**
             * PolychromeSprite contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.contentMask = null;

            /**
             * PolychromeSprite cornerRadii.
             * @member {zed.scene.ICorners|null|undefined} cornerRadii
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.cornerRadii = null;

            /**
             * PolychromeSprite tile.
             * @member {zed.scene.IAtlasTile|null|undefined} tile
             * @memberof zed.scene.PolychromeSprite
             * @instance
             */
            PolychromeSprite.prototype.tile = null;

            /**
             * Creates a new PolychromeSprite instance using the specified properties.
             * @function create
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {zed.scene.IPolychromeSprite=} [properties] Properties to set
             * @returns {zed.scene.PolychromeSprite} PolychromeSprite instance
             */
            PolychromeSprite.create = function create(properties) {
                return new PolychromeSprite(properties);
            };

            /**
             * Encodes the specified PolychromeSprite message. Does not implicitly {@link zed.scene.PolychromeSprite.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {zed.scene.IPolychromeSprite} message PolychromeSprite message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            PolychromeSprite.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.grayscale != null && Object.hasOwnProperty.call(message, "grayscale"))
                    writer.uint32(/* id 2, wireType 0 =*/16).bool(message.grayscale);
                if (message.opacity != null && Object.hasOwnProperty.call(message, "opacity"))
                    writer.uint32(/* id 3, wireType 5 =*/29).float(message.opacity);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.cornerRadii != null && Object.hasOwnProperty.call(message, "cornerRadii"))
                    $root.zed.scene.Corners.encode(message.cornerRadii, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                if (message.tile != null && Object.hasOwnProperty.call(message, "tile"))
                    $root.zed.scene.AtlasTile.encode(message.tile, writer.uint32(/* id 7, wireType 2 =*/58).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified PolychromeSprite message, length delimited. Does not implicitly {@link zed.scene.PolychromeSprite.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {zed.scene.IPolychromeSprite} message PolychromeSprite message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            PolychromeSprite.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a PolychromeSprite message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.PolychromeSprite} PolychromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            PolychromeSprite.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.PolychromeSprite();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.grayscale = reader.bool();
                            break;
                        }
                    case 3: {
                            message.opacity = reader.float();
                            break;
                        }
                    case 4: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 6: {
                            message.cornerRadii = $root.zed.scene.Corners.decode(reader, reader.uint32());
                            break;
                        }
                    case 7: {
                            message.tile = $root.zed.scene.AtlasTile.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a PolychromeSprite message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.PolychromeSprite} PolychromeSprite
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            PolychromeSprite.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a PolychromeSprite message.
             * @function verify
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            PolychromeSprite.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.grayscale != null && message.hasOwnProperty("grayscale"))
                    if (typeof message.grayscale !== "boolean")
                        return "grayscale: boolean expected";
                if (message.opacity != null && message.hasOwnProperty("opacity"))
                    if (typeof message.opacity !== "number")
                        return "opacity: number expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.cornerRadii != null && message.hasOwnProperty("cornerRadii")) {
                    let error = $root.zed.scene.Corners.verify(message.cornerRadii);
                    if (error)
                        return "cornerRadii." + error;
                }
                if (message.tile != null && message.hasOwnProperty("tile")) {
                    let error = $root.zed.scene.AtlasTile.verify(message.tile);
                    if (error)
                        return "tile." + error;
                }
                return null;
            };

            /**
             * Creates a PolychromeSprite message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.PolychromeSprite} PolychromeSprite
             */
            PolychromeSprite.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.PolychromeSprite)
                    return object;
                let message = new $root.zed.scene.PolychromeSprite();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.grayscale != null)
                    message.grayscale = Boolean(object.grayscale);
                if (object.opacity != null)
                    message.opacity = Number(object.opacity);
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.PolychromeSprite.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.PolychromeSprite.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.cornerRadii != null) {
                    if (typeof object.cornerRadii !== "object")
                        throw TypeError(".zed.scene.PolychromeSprite.cornerRadii: object expected");
                    message.cornerRadii = $root.zed.scene.Corners.fromObject(object.cornerRadii);
                }
                if (object.tile != null) {
                    if (typeof object.tile !== "object")
                        throw TypeError(".zed.scene.PolychromeSprite.tile: object expected");
                    message.tile = $root.zed.scene.AtlasTile.fromObject(object.tile);
                }
                return message;
            };

            /**
             * Creates a plain object from a PolychromeSprite message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {zed.scene.PolychromeSprite} message PolychromeSprite
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            PolychromeSprite.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.order = 0;
                    object.grayscale = false;
                    object.opacity = 0;
                    object.bounds = null;
                    object.contentMask = null;
                    object.cornerRadii = null;
                    object.tile = null;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.grayscale != null && message.hasOwnProperty("grayscale"))
                    object.grayscale = message.grayscale;
                if (message.opacity != null && message.hasOwnProperty("opacity"))
                    object.opacity = options.json && !isFinite(message.opacity) ? String(message.opacity) : message.opacity;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.cornerRadii != null && message.hasOwnProperty("cornerRadii"))
                    object.cornerRadii = $root.zed.scene.Corners.toObject(message.cornerRadii, options);
                if (message.tile != null && message.hasOwnProperty("tile"))
                    object.tile = $root.zed.scene.AtlasTile.toObject(message.tile, options);
                return object;
            };

            /**
             * Converts this PolychromeSprite to JSON.
             * @function toJSON
             * @memberof zed.scene.PolychromeSprite
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            PolychromeSprite.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for PolychromeSprite
             * @function getTypeUrl
             * @memberof zed.scene.PolychromeSprite
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            PolychromeSprite.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.PolychromeSprite";
            };

            return PolychromeSprite;
        })();

        scene.PathVertex = (function() {

            /**
             * Properties of a PathVertex.
             * @memberof zed.scene
             * @interface IPathVertex
             * @property {zed.scene.IPoint|null} [xyPosition] PathVertex xyPosition
             * @property {zed.scene.IPoint|null} [stPosition] PathVertex stPosition
             * @property {zed.scene.IContentMask|null} [contentMask] PathVertex contentMask
             */

            /**
             * Constructs a new PathVertex.
             * @memberof zed.scene
             * @classdesc Represents a PathVertex.
             * @implements IPathVertex
             * @constructor
             * @param {zed.scene.IPathVertex=} [properties] Properties to set
             */
            function PathVertex(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * PathVertex xyPosition.
             * @member {zed.scene.IPoint|null|undefined} xyPosition
             * @memberof zed.scene.PathVertex
             * @instance
             */
            PathVertex.prototype.xyPosition = null;

            /**
             * PathVertex stPosition.
             * @member {zed.scene.IPoint|null|undefined} stPosition
             * @memberof zed.scene.PathVertex
             * @instance
             */
            PathVertex.prototype.stPosition = null;

            /**
             * PathVertex contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.PathVertex
             * @instance
             */
            PathVertex.prototype.contentMask = null;

            /**
             * Creates a new PathVertex instance using the specified properties.
             * @function create
             * @memberof zed.scene.PathVertex
             * @static
             * @param {zed.scene.IPathVertex=} [properties] Properties to set
             * @returns {zed.scene.PathVertex} PathVertex instance
             */
            PathVertex.create = function create(properties) {
                return new PathVertex(properties);
            };

            /**
             * Encodes the specified PathVertex message. Does not implicitly {@link zed.scene.PathVertex.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.PathVertex
             * @static
             * @param {zed.scene.IPathVertex} message PathVertex message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            PathVertex.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.xyPosition != null && Object.hasOwnProperty.call(message, "xyPosition"))
                    $root.zed.scene.Point.encode(message.xyPosition, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.stPosition != null && Object.hasOwnProperty.call(message, "stPosition"))
                    $root.zed.scene.Point.encode(message.stPosition, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified PathVertex message, length delimited. Does not implicitly {@link zed.scene.PathVertex.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.PathVertex
             * @static
             * @param {zed.scene.IPathVertex} message PathVertex message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            PathVertex.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a PathVertex message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.PathVertex
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.PathVertex} PathVertex
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            PathVertex.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.PathVertex();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.xyPosition = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.stPosition = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a PathVertex message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.PathVertex
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.PathVertex} PathVertex
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            PathVertex.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a PathVertex message.
             * @function verify
             * @memberof zed.scene.PathVertex
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            PathVertex.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.xyPosition != null && message.hasOwnProperty("xyPosition")) {
                    let error = $root.zed.scene.Point.verify(message.xyPosition);
                    if (error)
                        return "xyPosition." + error;
                }
                if (message.stPosition != null && message.hasOwnProperty("stPosition")) {
                    let error = $root.zed.scene.Point.verify(message.stPosition);
                    if (error)
                        return "stPosition." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                return null;
            };

            /**
             * Creates a PathVertex message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.PathVertex
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.PathVertex} PathVertex
             */
            PathVertex.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.PathVertex)
                    return object;
                let message = new $root.zed.scene.PathVertex();
                if (object.xyPosition != null) {
                    if (typeof object.xyPosition !== "object")
                        throw TypeError(".zed.scene.PathVertex.xyPosition: object expected");
                    message.xyPosition = $root.zed.scene.Point.fromObject(object.xyPosition);
                }
                if (object.stPosition != null) {
                    if (typeof object.stPosition !== "object")
                        throw TypeError(".zed.scene.PathVertex.stPosition: object expected");
                    message.stPosition = $root.zed.scene.Point.fromObject(object.stPosition);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.PathVertex.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                return message;
            };

            /**
             * Creates a plain object from a PathVertex message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.PathVertex
             * @static
             * @param {zed.scene.PathVertex} message PathVertex
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            PathVertex.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.xyPosition = null;
                    object.stPosition = null;
                    object.contentMask = null;
                }
                if (message.xyPosition != null && message.hasOwnProperty("xyPosition"))
                    object.xyPosition = $root.zed.scene.Point.toObject(message.xyPosition, options);
                if (message.stPosition != null && message.hasOwnProperty("stPosition"))
                    object.stPosition = $root.zed.scene.Point.toObject(message.stPosition, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                return object;
            };

            /**
             * Converts this PathVertex to JSON.
             * @function toJSON
             * @memberof zed.scene.PathVertex
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            PathVertex.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for PathVertex
             * @function getTypeUrl
             * @memberof zed.scene.PathVertex
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            PathVertex.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.PathVertex";
            };

            return PathVertex;
        })();

        scene.Path = (function() {

            /**
             * Properties of a Path.
             * @memberof zed.scene
             * @interface IPath
             * @property {number|null} [order] Path order
             * @property {zed.scene.IBounds|null} [bounds] Path bounds
             * @property {zed.scene.IContentMask|null} [contentMask] Path contentMask
             * @property {zed.scene.IBackground|null} [color] Path color
             * @property {Array.<zed.scene.IPathVertex>|null} [vertices] Path vertices
             */

            /**
             * Constructs a new Path.
             * @memberof zed.scene
             * @classdesc Represents a Path.
             * @implements IPath
             * @constructor
             * @param {zed.scene.IPath=} [properties] Properties to set
             */
            function Path(properties) {
                this.vertices = [];
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Path order.
             * @member {number} order
             * @memberof zed.scene.Path
             * @instance
             */
            Path.prototype.order = 0;

            /**
             * Path bounds.
             * @member {zed.scene.IBounds|null|undefined} bounds
             * @memberof zed.scene.Path
             * @instance
             */
            Path.prototype.bounds = null;

            /**
             * Path contentMask.
             * @member {zed.scene.IContentMask|null|undefined} contentMask
             * @memberof zed.scene.Path
             * @instance
             */
            Path.prototype.contentMask = null;

            /**
             * Path color.
             * @member {zed.scene.IBackground|null|undefined} color
             * @memberof zed.scene.Path
             * @instance
             */
            Path.prototype.color = null;

            /**
             * Path vertices.
             * @member {Array.<zed.scene.IPathVertex>} vertices
             * @memberof zed.scene.Path
             * @instance
             */
            Path.prototype.vertices = $util.emptyArray;

            /**
             * Creates a new Path instance using the specified properties.
             * @function create
             * @memberof zed.scene.Path
             * @static
             * @param {zed.scene.IPath=} [properties] Properties to set
             * @returns {zed.scene.Path} Path instance
             */
            Path.create = function create(properties) {
                return new Path(properties);
            };

            /**
             * Encodes the specified Path message. Does not implicitly {@link zed.scene.Path.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Path
             * @static
             * @param {zed.scene.IPath} message Path message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Path.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.order != null && Object.hasOwnProperty.call(message, "order"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.order);
                if (message.bounds != null && Object.hasOwnProperty.call(message, "bounds"))
                    $root.zed.scene.Bounds.encode(message.bounds, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.contentMask != null && Object.hasOwnProperty.call(message, "contentMask"))
                    $root.zed.scene.ContentMask.encode(message.contentMask, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.color != null && Object.hasOwnProperty.call(message, "color"))
                    $root.zed.scene.Background.encode(message.color, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.vertices != null && message.vertices.length)
                    for (let i = 0; i < message.vertices.length; ++i)
                        $root.zed.scene.PathVertex.encode(message.vertices[i], writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified Path message, length delimited. Does not implicitly {@link zed.scene.Path.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Path
             * @static
             * @param {zed.scene.IPath} message Path message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Path.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Path message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Path
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Path} Path
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Path.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Path();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.order = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.bounds = $root.zed.scene.Bounds.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.contentMask = $root.zed.scene.ContentMask.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.color = $root.zed.scene.Background.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            if (!(message.vertices && message.vertices.length))
                                message.vertices = [];
                            message.vertices.push($root.zed.scene.PathVertex.decode(reader, reader.uint32()));
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Path message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Path
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Path} Path
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Path.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Path message.
             * @function verify
             * @memberof zed.scene.Path
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Path.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.order != null && message.hasOwnProperty("order"))
                    if (!$util.isInteger(message.order))
                        return "order: integer expected";
                if (message.bounds != null && message.hasOwnProperty("bounds")) {
                    let error = $root.zed.scene.Bounds.verify(message.bounds);
                    if (error)
                        return "bounds." + error;
                }
                if (message.contentMask != null && message.hasOwnProperty("contentMask")) {
                    let error = $root.zed.scene.ContentMask.verify(message.contentMask);
                    if (error)
                        return "contentMask." + error;
                }
                if (message.color != null && message.hasOwnProperty("color")) {
                    let error = $root.zed.scene.Background.verify(message.color);
                    if (error)
                        return "color." + error;
                }
                if (message.vertices != null && message.hasOwnProperty("vertices")) {
                    if (!Array.isArray(message.vertices))
                        return "vertices: array expected";
                    for (let i = 0; i < message.vertices.length; ++i) {
                        let error = $root.zed.scene.PathVertex.verify(message.vertices[i]);
                        if (error)
                            return "vertices." + error;
                    }
                }
                return null;
            };

            /**
             * Creates a Path message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Path
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Path} Path
             */
            Path.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Path)
                    return object;
                let message = new $root.zed.scene.Path();
                if (object.order != null)
                    message.order = object.order >>> 0;
                if (object.bounds != null) {
                    if (typeof object.bounds !== "object")
                        throw TypeError(".zed.scene.Path.bounds: object expected");
                    message.bounds = $root.zed.scene.Bounds.fromObject(object.bounds);
                }
                if (object.contentMask != null) {
                    if (typeof object.contentMask !== "object")
                        throw TypeError(".zed.scene.Path.contentMask: object expected");
                    message.contentMask = $root.zed.scene.ContentMask.fromObject(object.contentMask);
                }
                if (object.color != null) {
                    if (typeof object.color !== "object")
                        throw TypeError(".zed.scene.Path.color: object expected");
                    message.color = $root.zed.scene.Background.fromObject(object.color);
                }
                if (object.vertices) {
                    if (!Array.isArray(object.vertices))
                        throw TypeError(".zed.scene.Path.vertices: array expected");
                    message.vertices = [];
                    for (let i = 0; i < object.vertices.length; ++i) {
                        if (typeof object.vertices[i] !== "object")
                            throw TypeError(".zed.scene.Path.vertices: object expected");
                        message.vertices[i] = $root.zed.scene.PathVertex.fromObject(object.vertices[i]);
                    }
                }
                return message;
            };

            /**
             * Creates a plain object from a Path message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Path
             * @static
             * @param {zed.scene.Path} message Path
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Path.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.arrays || options.defaults)
                    object.vertices = [];
                if (options.defaults) {
                    object.order = 0;
                    object.bounds = null;
                    object.contentMask = null;
                    object.color = null;
                }
                if (message.order != null && message.hasOwnProperty("order"))
                    object.order = message.order;
                if (message.bounds != null && message.hasOwnProperty("bounds"))
                    object.bounds = $root.zed.scene.Bounds.toObject(message.bounds, options);
                if (message.contentMask != null && message.hasOwnProperty("contentMask"))
                    object.contentMask = $root.zed.scene.ContentMask.toObject(message.contentMask, options);
                if (message.color != null && message.hasOwnProperty("color"))
                    object.color = $root.zed.scene.Background.toObject(message.color, options);
                if (message.vertices && message.vertices.length) {
                    object.vertices = [];
                    for (let j = 0; j < message.vertices.length; ++j)
                        object.vertices[j] = $root.zed.scene.PathVertex.toObject(message.vertices[j], options);
                }
                return object;
            };

            /**
             * Converts this Path to JSON.
             * @function toJSON
             * @memberof zed.scene.Path
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Path.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Path
             * @function getTypeUrl
             * @memberof zed.scene.Path
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Path.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Path";
            };

            return Path;
        })();

        scene.InputMessage = (function() {

            /**
             * Properties of an InputMessage.
             * @memberof zed.scene
             * @interface IInputMessage
             * @property {zed.scene.IMouseMoveInput|null} [mouseMove] InputMessage mouseMove
             * @property {zed.scene.IMouseDownInput|null} [mouseDown] InputMessage mouseDown
             * @property {zed.scene.IMouseUpInput|null} [mouseUp] InputMessage mouseUp
             * @property {zed.scene.IScrollInput|null} [scroll] InputMessage scroll
             * @property {zed.scene.IKeyDownInput|null} [keyDown] InputMessage keyDown
             * @property {zed.scene.IKeyUpInput|null} [keyUp] InputMessage keyUp
             * @property {zed.scene.IResizeInput|null} [resize] InputMessage resize
             */

            /**
             * Constructs a new InputMessage.
             * @memberof zed.scene
             * @classdesc Represents an InputMessage.
             * @implements IInputMessage
             * @constructor
             * @param {zed.scene.IInputMessage=} [properties] Properties to set
             */
            function InputMessage(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * InputMessage mouseMove.
             * @member {zed.scene.IMouseMoveInput|null|undefined} mouseMove
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.mouseMove = null;

            /**
             * InputMessage mouseDown.
             * @member {zed.scene.IMouseDownInput|null|undefined} mouseDown
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.mouseDown = null;

            /**
             * InputMessage mouseUp.
             * @member {zed.scene.IMouseUpInput|null|undefined} mouseUp
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.mouseUp = null;

            /**
             * InputMessage scroll.
             * @member {zed.scene.IScrollInput|null|undefined} scroll
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.scroll = null;

            /**
             * InputMessage keyDown.
             * @member {zed.scene.IKeyDownInput|null|undefined} keyDown
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.keyDown = null;

            /**
             * InputMessage keyUp.
             * @member {zed.scene.IKeyUpInput|null|undefined} keyUp
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.keyUp = null;

            /**
             * InputMessage resize.
             * @member {zed.scene.IResizeInput|null|undefined} resize
             * @memberof zed.scene.InputMessage
             * @instance
             */
            InputMessage.prototype.resize = null;

            // OneOf field names bound to virtual getters and setters
            let $oneOfFields;

            /**
             * InputMessage kind.
             * @member {"mouseMove"|"mouseDown"|"mouseUp"|"scroll"|"keyDown"|"keyUp"|"resize"|undefined} kind
             * @memberof zed.scene.InputMessage
             * @instance
             */
            Object.defineProperty(InputMessage.prototype, "kind", {
                get: $util.oneOfGetter($oneOfFields = ["mouseMove", "mouseDown", "mouseUp", "scroll", "keyDown", "keyUp", "resize"]),
                set: $util.oneOfSetter($oneOfFields)
            });

            /**
             * Creates a new InputMessage instance using the specified properties.
             * @function create
             * @memberof zed.scene.InputMessage
             * @static
             * @param {zed.scene.IInputMessage=} [properties] Properties to set
             * @returns {zed.scene.InputMessage} InputMessage instance
             */
            InputMessage.create = function create(properties) {
                return new InputMessage(properties);
            };

            /**
             * Encodes the specified InputMessage message. Does not implicitly {@link zed.scene.InputMessage.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.InputMessage
             * @static
             * @param {zed.scene.IInputMessage} message InputMessage message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            InputMessage.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.mouseMove != null && Object.hasOwnProperty.call(message, "mouseMove"))
                    $root.zed.scene.MouseMoveInput.encode(message.mouseMove, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.mouseDown != null && Object.hasOwnProperty.call(message, "mouseDown"))
                    $root.zed.scene.MouseDownInput.encode(message.mouseDown, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.mouseUp != null && Object.hasOwnProperty.call(message, "mouseUp"))
                    $root.zed.scene.MouseUpInput.encode(message.mouseUp, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.scroll != null && Object.hasOwnProperty.call(message, "scroll"))
                    $root.zed.scene.ScrollInput.encode(message.scroll, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                if (message.keyDown != null && Object.hasOwnProperty.call(message, "keyDown"))
                    $root.zed.scene.KeyDownInput.encode(message.keyDown, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                if (message.keyUp != null && Object.hasOwnProperty.call(message, "keyUp"))
                    $root.zed.scene.KeyUpInput.encode(message.keyUp, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                if (message.resize != null && Object.hasOwnProperty.call(message, "resize"))
                    $root.zed.scene.ResizeInput.encode(message.resize, writer.uint32(/* id 7, wireType 2 =*/58).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified InputMessage message, length delimited. Does not implicitly {@link zed.scene.InputMessage.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.InputMessage
             * @static
             * @param {zed.scene.IInputMessage} message InputMessage message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            InputMessage.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an InputMessage message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.InputMessage
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.InputMessage} InputMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            InputMessage.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.InputMessage();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.mouseMove = $root.zed.scene.MouseMoveInput.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.mouseDown = $root.zed.scene.MouseDownInput.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.mouseUp = $root.zed.scene.MouseUpInput.decode(reader, reader.uint32());
                            break;
                        }
                    case 4: {
                            message.scroll = $root.zed.scene.ScrollInput.decode(reader, reader.uint32());
                            break;
                        }
                    case 5: {
                            message.keyDown = $root.zed.scene.KeyDownInput.decode(reader, reader.uint32());
                            break;
                        }
                    case 6: {
                            message.keyUp = $root.zed.scene.KeyUpInput.decode(reader, reader.uint32());
                            break;
                        }
                    case 7: {
                            message.resize = $root.zed.scene.ResizeInput.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an InputMessage message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.InputMessage
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.InputMessage} InputMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            InputMessage.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an InputMessage message.
             * @function verify
             * @memberof zed.scene.InputMessage
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            InputMessage.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                let properties = {};
                if (message.mouseMove != null && message.hasOwnProperty("mouseMove")) {
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.MouseMoveInput.verify(message.mouseMove);
                        if (error)
                            return "mouseMove." + error;
                    }
                }
                if (message.mouseDown != null && message.hasOwnProperty("mouseDown")) {
                    if (properties.kind === 1)
                        return "kind: multiple values";
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.MouseDownInput.verify(message.mouseDown);
                        if (error)
                            return "mouseDown." + error;
                    }
                }
                if (message.mouseUp != null && message.hasOwnProperty("mouseUp")) {
                    if (properties.kind === 1)
                        return "kind: multiple values";
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.MouseUpInput.verify(message.mouseUp);
                        if (error)
                            return "mouseUp." + error;
                    }
                }
                if (message.scroll != null && message.hasOwnProperty("scroll")) {
                    if (properties.kind === 1)
                        return "kind: multiple values";
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.ScrollInput.verify(message.scroll);
                        if (error)
                            return "scroll." + error;
                    }
                }
                if (message.keyDown != null && message.hasOwnProperty("keyDown")) {
                    if (properties.kind === 1)
                        return "kind: multiple values";
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.KeyDownInput.verify(message.keyDown);
                        if (error)
                            return "keyDown." + error;
                    }
                }
                if (message.keyUp != null && message.hasOwnProperty("keyUp")) {
                    if (properties.kind === 1)
                        return "kind: multiple values";
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.KeyUpInput.verify(message.keyUp);
                        if (error)
                            return "keyUp." + error;
                    }
                }
                if (message.resize != null && message.hasOwnProperty("resize")) {
                    if (properties.kind === 1)
                        return "kind: multiple values";
                    properties.kind = 1;
                    {
                        let error = $root.zed.scene.ResizeInput.verify(message.resize);
                        if (error)
                            return "resize." + error;
                    }
                }
                return null;
            };

            /**
             * Creates an InputMessage message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.InputMessage
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.InputMessage} InputMessage
             */
            InputMessage.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.InputMessage)
                    return object;
                let message = new $root.zed.scene.InputMessage();
                if (object.mouseMove != null) {
                    if (typeof object.mouseMove !== "object")
                        throw TypeError(".zed.scene.InputMessage.mouseMove: object expected");
                    message.mouseMove = $root.zed.scene.MouseMoveInput.fromObject(object.mouseMove);
                }
                if (object.mouseDown != null) {
                    if (typeof object.mouseDown !== "object")
                        throw TypeError(".zed.scene.InputMessage.mouseDown: object expected");
                    message.mouseDown = $root.zed.scene.MouseDownInput.fromObject(object.mouseDown);
                }
                if (object.mouseUp != null) {
                    if (typeof object.mouseUp !== "object")
                        throw TypeError(".zed.scene.InputMessage.mouseUp: object expected");
                    message.mouseUp = $root.zed.scene.MouseUpInput.fromObject(object.mouseUp);
                }
                if (object.scroll != null) {
                    if (typeof object.scroll !== "object")
                        throw TypeError(".zed.scene.InputMessage.scroll: object expected");
                    message.scroll = $root.zed.scene.ScrollInput.fromObject(object.scroll);
                }
                if (object.keyDown != null) {
                    if (typeof object.keyDown !== "object")
                        throw TypeError(".zed.scene.InputMessage.keyDown: object expected");
                    message.keyDown = $root.zed.scene.KeyDownInput.fromObject(object.keyDown);
                }
                if (object.keyUp != null) {
                    if (typeof object.keyUp !== "object")
                        throw TypeError(".zed.scene.InputMessage.keyUp: object expected");
                    message.keyUp = $root.zed.scene.KeyUpInput.fromObject(object.keyUp);
                }
                if (object.resize != null) {
                    if (typeof object.resize !== "object")
                        throw TypeError(".zed.scene.InputMessage.resize: object expected");
                    message.resize = $root.zed.scene.ResizeInput.fromObject(object.resize);
                }
                return message;
            };

            /**
             * Creates a plain object from an InputMessage message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.InputMessage
             * @static
             * @param {zed.scene.InputMessage} message InputMessage
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            InputMessage.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (message.mouseMove != null && message.hasOwnProperty("mouseMove")) {
                    object.mouseMove = $root.zed.scene.MouseMoveInput.toObject(message.mouseMove, options);
                    if (options.oneofs)
                        object.kind = "mouseMove";
                }
                if (message.mouseDown != null && message.hasOwnProperty("mouseDown")) {
                    object.mouseDown = $root.zed.scene.MouseDownInput.toObject(message.mouseDown, options);
                    if (options.oneofs)
                        object.kind = "mouseDown";
                }
                if (message.mouseUp != null && message.hasOwnProperty("mouseUp")) {
                    object.mouseUp = $root.zed.scene.MouseUpInput.toObject(message.mouseUp, options);
                    if (options.oneofs)
                        object.kind = "mouseUp";
                }
                if (message.scroll != null && message.hasOwnProperty("scroll")) {
                    object.scroll = $root.zed.scene.ScrollInput.toObject(message.scroll, options);
                    if (options.oneofs)
                        object.kind = "scroll";
                }
                if (message.keyDown != null && message.hasOwnProperty("keyDown")) {
                    object.keyDown = $root.zed.scene.KeyDownInput.toObject(message.keyDown, options);
                    if (options.oneofs)
                        object.kind = "keyDown";
                }
                if (message.keyUp != null && message.hasOwnProperty("keyUp")) {
                    object.keyUp = $root.zed.scene.KeyUpInput.toObject(message.keyUp, options);
                    if (options.oneofs)
                        object.kind = "keyUp";
                }
                if (message.resize != null && message.hasOwnProperty("resize")) {
                    object.resize = $root.zed.scene.ResizeInput.toObject(message.resize, options);
                    if (options.oneofs)
                        object.kind = "resize";
                }
                return object;
            };

            /**
             * Converts this InputMessage to JSON.
             * @function toJSON
             * @memberof zed.scene.InputMessage
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            InputMessage.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for InputMessage
             * @function getTypeUrl
             * @memberof zed.scene.InputMessage
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            InputMessage.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.InputMessage";
            };

            return InputMessage;
        })();

        scene.Modifiers = (function() {

            /**
             * Properties of a Modifiers.
             * @memberof zed.scene
             * @interface IModifiers
             * @property {boolean|null} [control] Modifiers control
             * @property {boolean|null} [alt] Modifiers alt
             * @property {boolean|null} [shift] Modifiers shift
             * @property {boolean|null} [meta] Modifiers meta
             */

            /**
             * Constructs a new Modifiers.
             * @memberof zed.scene
             * @classdesc Represents a Modifiers.
             * @implements IModifiers
             * @constructor
             * @param {zed.scene.IModifiers=} [properties] Properties to set
             */
            function Modifiers(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Modifiers control.
             * @member {boolean} control
             * @memberof zed.scene.Modifiers
             * @instance
             */
            Modifiers.prototype.control = false;

            /**
             * Modifiers alt.
             * @member {boolean} alt
             * @memberof zed.scene.Modifiers
             * @instance
             */
            Modifiers.prototype.alt = false;

            /**
             * Modifiers shift.
             * @member {boolean} shift
             * @memberof zed.scene.Modifiers
             * @instance
             */
            Modifiers.prototype.shift = false;

            /**
             * Modifiers meta.
             * @member {boolean} meta
             * @memberof zed.scene.Modifiers
             * @instance
             */
            Modifiers.prototype.meta = false;

            /**
             * Creates a new Modifiers instance using the specified properties.
             * @function create
             * @memberof zed.scene.Modifiers
             * @static
             * @param {zed.scene.IModifiers=} [properties] Properties to set
             * @returns {zed.scene.Modifiers} Modifiers instance
             */
            Modifiers.create = function create(properties) {
                return new Modifiers(properties);
            };

            /**
             * Encodes the specified Modifiers message. Does not implicitly {@link zed.scene.Modifiers.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.Modifiers
             * @static
             * @param {zed.scene.IModifiers} message Modifiers message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Modifiers.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.control != null && Object.hasOwnProperty.call(message, "control"))
                    writer.uint32(/* id 1, wireType 0 =*/8).bool(message.control);
                if (message.alt != null && Object.hasOwnProperty.call(message, "alt"))
                    writer.uint32(/* id 2, wireType 0 =*/16).bool(message.alt);
                if (message.shift != null && Object.hasOwnProperty.call(message, "shift"))
                    writer.uint32(/* id 3, wireType 0 =*/24).bool(message.shift);
                if (message.meta != null && Object.hasOwnProperty.call(message, "meta"))
                    writer.uint32(/* id 4, wireType 0 =*/32).bool(message.meta);
                return writer;
            };

            /**
             * Encodes the specified Modifiers message, length delimited. Does not implicitly {@link zed.scene.Modifiers.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.Modifiers
             * @static
             * @param {zed.scene.IModifiers} message Modifiers message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Modifiers.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Modifiers message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.Modifiers
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.Modifiers} Modifiers
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Modifiers.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.Modifiers();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.control = reader.bool();
                            break;
                        }
                    case 2: {
                            message.alt = reader.bool();
                            break;
                        }
                    case 3: {
                            message.shift = reader.bool();
                            break;
                        }
                    case 4: {
                            message.meta = reader.bool();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Modifiers message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.Modifiers
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.Modifiers} Modifiers
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Modifiers.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Modifiers message.
             * @function verify
             * @memberof zed.scene.Modifiers
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Modifiers.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.control != null && message.hasOwnProperty("control"))
                    if (typeof message.control !== "boolean")
                        return "control: boolean expected";
                if (message.alt != null && message.hasOwnProperty("alt"))
                    if (typeof message.alt !== "boolean")
                        return "alt: boolean expected";
                if (message.shift != null && message.hasOwnProperty("shift"))
                    if (typeof message.shift !== "boolean")
                        return "shift: boolean expected";
                if (message.meta != null && message.hasOwnProperty("meta"))
                    if (typeof message.meta !== "boolean")
                        return "meta: boolean expected";
                return null;
            };

            /**
             * Creates a Modifiers message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.Modifiers
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.Modifiers} Modifiers
             */
            Modifiers.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.Modifiers)
                    return object;
                let message = new $root.zed.scene.Modifiers();
                if (object.control != null)
                    message.control = Boolean(object.control);
                if (object.alt != null)
                    message.alt = Boolean(object.alt);
                if (object.shift != null)
                    message.shift = Boolean(object.shift);
                if (object.meta != null)
                    message.meta = Boolean(object.meta);
                return message;
            };

            /**
             * Creates a plain object from a Modifiers message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.Modifiers
             * @static
             * @param {zed.scene.Modifiers} message Modifiers
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Modifiers.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.control = false;
                    object.alt = false;
                    object.shift = false;
                    object.meta = false;
                }
                if (message.control != null && message.hasOwnProperty("control"))
                    object.control = message.control;
                if (message.alt != null && message.hasOwnProperty("alt"))
                    object.alt = message.alt;
                if (message.shift != null && message.hasOwnProperty("shift"))
                    object.shift = message.shift;
                if (message.meta != null && message.hasOwnProperty("meta"))
                    object.meta = message.meta;
                return object;
            };

            /**
             * Converts this Modifiers to JSON.
             * @function toJSON
             * @memberof zed.scene.Modifiers
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Modifiers.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for Modifiers
             * @function getTypeUrl
             * @memberof zed.scene.Modifiers
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Modifiers.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.Modifiers";
            };

            return Modifiers;
        })();

        scene.MouseMoveInput = (function() {

            /**
             * Properties of a MouseMoveInput.
             * @memberof zed.scene
             * @interface IMouseMoveInput
             * @property {zed.scene.IPoint|null} [position] MouseMoveInput position
             * @property {zed.scene.IModifiers|null} [modifiers] MouseMoveInput modifiers
             */

            /**
             * Constructs a new MouseMoveInput.
             * @memberof zed.scene
             * @classdesc Represents a MouseMoveInput.
             * @implements IMouseMoveInput
             * @constructor
             * @param {zed.scene.IMouseMoveInput=} [properties] Properties to set
             */
            function MouseMoveInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * MouseMoveInput position.
             * @member {zed.scene.IPoint|null|undefined} position
             * @memberof zed.scene.MouseMoveInput
             * @instance
             */
            MouseMoveInput.prototype.position = null;

            /**
             * MouseMoveInput modifiers.
             * @member {zed.scene.IModifiers|null|undefined} modifiers
             * @memberof zed.scene.MouseMoveInput
             * @instance
             */
            MouseMoveInput.prototype.modifiers = null;

            /**
             * Creates a new MouseMoveInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {zed.scene.IMouseMoveInput=} [properties] Properties to set
             * @returns {zed.scene.MouseMoveInput} MouseMoveInput instance
             */
            MouseMoveInput.create = function create(properties) {
                return new MouseMoveInput(properties);
            };

            /**
             * Encodes the specified MouseMoveInput message. Does not implicitly {@link zed.scene.MouseMoveInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {zed.scene.IMouseMoveInput} message MouseMoveInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MouseMoveInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.position != null && Object.hasOwnProperty.call(message, "position"))
                    $root.zed.scene.Point.encode(message.position, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.modifiers != null && Object.hasOwnProperty.call(message, "modifiers"))
                    $root.zed.scene.Modifiers.encode(message.modifiers, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified MouseMoveInput message, length delimited. Does not implicitly {@link zed.scene.MouseMoveInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {zed.scene.IMouseMoveInput} message MouseMoveInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MouseMoveInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a MouseMoveInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.MouseMoveInput} MouseMoveInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MouseMoveInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.MouseMoveInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.position = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.modifiers = $root.zed.scene.Modifiers.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a MouseMoveInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.MouseMoveInput} MouseMoveInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MouseMoveInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a MouseMoveInput message.
             * @function verify
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            MouseMoveInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.position != null && message.hasOwnProperty("position")) {
                    let error = $root.zed.scene.Point.verify(message.position);
                    if (error)
                        return "position." + error;
                }
                if (message.modifiers != null && message.hasOwnProperty("modifiers")) {
                    let error = $root.zed.scene.Modifiers.verify(message.modifiers);
                    if (error)
                        return "modifiers." + error;
                }
                return null;
            };

            /**
             * Creates a MouseMoveInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.MouseMoveInput} MouseMoveInput
             */
            MouseMoveInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.MouseMoveInput)
                    return object;
                let message = new $root.zed.scene.MouseMoveInput();
                if (object.position != null) {
                    if (typeof object.position !== "object")
                        throw TypeError(".zed.scene.MouseMoveInput.position: object expected");
                    message.position = $root.zed.scene.Point.fromObject(object.position);
                }
                if (object.modifiers != null) {
                    if (typeof object.modifiers !== "object")
                        throw TypeError(".zed.scene.MouseMoveInput.modifiers: object expected");
                    message.modifiers = $root.zed.scene.Modifiers.fromObject(object.modifiers);
                }
                return message;
            };

            /**
             * Creates a plain object from a MouseMoveInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {zed.scene.MouseMoveInput} message MouseMoveInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            MouseMoveInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.position = null;
                    object.modifiers = null;
                }
                if (message.position != null && message.hasOwnProperty("position"))
                    object.position = $root.zed.scene.Point.toObject(message.position, options);
                if (message.modifiers != null && message.hasOwnProperty("modifiers"))
                    object.modifiers = $root.zed.scene.Modifiers.toObject(message.modifiers, options);
                return object;
            };

            /**
             * Converts this MouseMoveInput to JSON.
             * @function toJSON
             * @memberof zed.scene.MouseMoveInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            MouseMoveInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for MouseMoveInput
             * @function getTypeUrl
             * @memberof zed.scene.MouseMoveInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            MouseMoveInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.MouseMoveInput";
            };

            return MouseMoveInput;
        })();

        scene.MouseDownInput = (function() {

            /**
             * Properties of a MouseDownInput.
             * @memberof zed.scene
             * @interface IMouseDownInput
             * @property {number|null} [button] MouseDownInput button
             * @property {zed.scene.IPoint|null} [position] MouseDownInput position
             * @property {number|null} [clickCount] MouseDownInput clickCount
             * @property {zed.scene.IModifiers|null} [modifiers] MouseDownInput modifiers
             */

            /**
             * Constructs a new MouseDownInput.
             * @memberof zed.scene
             * @classdesc Represents a MouseDownInput.
             * @implements IMouseDownInput
             * @constructor
             * @param {zed.scene.IMouseDownInput=} [properties] Properties to set
             */
            function MouseDownInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * MouseDownInput button.
             * @member {number} button
             * @memberof zed.scene.MouseDownInput
             * @instance
             */
            MouseDownInput.prototype.button = 0;

            /**
             * MouseDownInput position.
             * @member {zed.scene.IPoint|null|undefined} position
             * @memberof zed.scene.MouseDownInput
             * @instance
             */
            MouseDownInput.prototype.position = null;

            /**
             * MouseDownInput clickCount.
             * @member {number} clickCount
             * @memberof zed.scene.MouseDownInput
             * @instance
             */
            MouseDownInput.prototype.clickCount = 0;

            /**
             * MouseDownInput modifiers.
             * @member {zed.scene.IModifiers|null|undefined} modifiers
             * @memberof zed.scene.MouseDownInput
             * @instance
             */
            MouseDownInput.prototype.modifiers = null;

            /**
             * Creates a new MouseDownInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {zed.scene.IMouseDownInput=} [properties] Properties to set
             * @returns {zed.scene.MouseDownInput} MouseDownInput instance
             */
            MouseDownInput.create = function create(properties) {
                return new MouseDownInput(properties);
            };

            /**
             * Encodes the specified MouseDownInput message. Does not implicitly {@link zed.scene.MouseDownInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {zed.scene.IMouseDownInput} message MouseDownInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MouseDownInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.button != null && Object.hasOwnProperty.call(message, "button"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.button);
                if (message.position != null && Object.hasOwnProperty.call(message, "position"))
                    $root.zed.scene.Point.encode(message.position, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.clickCount != null && Object.hasOwnProperty.call(message, "clickCount"))
                    writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.clickCount);
                if (message.modifiers != null && Object.hasOwnProperty.call(message, "modifiers"))
                    $root.zed.scene.Modifiers.encode(message.modifiers, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified MouseDownInput message, length delimited. Does not implicitly {@link zed.scene.MouseDownInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {zed.scene.IMouseDownInput} message MouseDownInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MouseDownInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a MouseDownInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.MouseDownInput} MouseDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MouseDownInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.MouseDownInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.button = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.position = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.clickCount = reader.uint32();
                            break;
                        }
                    case 4: {
                            message.modifiers = $root.zed.scene.Modifiers.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a MouseDownInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.MouseDownInput} MouseDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MouseDownInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a MouseDownInput message.
             * @function verify
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            MouseDownInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.button != null && message.hasOwnProperty("button"))
                    if (!$util.isInteger(message.button))
                        return "button: integer expected";
                if (message.position != null && message.hasOwnProperty("position")) {
                    let error = $root.zed.scene.Point.verify(message.position);
                    if (error)
                        return "position." + error;
                }
                if (message.clickCount != null && message.hasOwnProperty("clickCount"))
                    if (!$util.isInteger(message.clickCount))
                        return "clickCount: integer expected";
                if (message.modifiers != null && message.hasOwnProperty("modifiers")) {
                    let error = $root.zed.scene.Modifiers.verify(message.modifiers);
                    if (error)
                        return "modifiers." + error;
                }
                return null;
            };

            /**
             * Creates a MouseDownInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.MouseDownInput} MouseDownInput
             */
            MouseDownInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.MouseDownInput)
                    return object;
                let message = new $root.zed.scene.MouseDownInput();
                if (object.button != null)
                    message.button = object.button >>> 0;
                if (object.position != null) {
                    if (typeof object.position !== "object")
                        throw TypeError(".zed.scene.MouseDownInput.position: object expected");
                    message.position = $root.zed.scene.Point.fromObject(object.position);
                }
                if (object.clickCount != null)
                    message.clickCount = object.clickCount >>> 0;
                if (object.modifiers != null) {
                    if (typeof object.modifiers !== "object")
                        throw TypeError(".zed.scene.MouseDownInput.modifiers: object expected");
                    message.modifiers = $root.zed.scene.Modifiers.fromObject(object.modifiers);
                }
                return message;
            };

            /**
             * Creates a plain object from a MouseDownInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {zed.scene.MouseDownInput} message MouseDownInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            MouseDownInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.button = 0;
                    object.position = null;
                    object.clickCount = 0;
                    object.modifiers = null;
                }
                if (message.button != null && message.hasOwnProperty("button"))
                    object.button = message.button;
                if (message.position != null && message.hasOwnProperty("position"))
                    object.position = $root.zed.scene.Point.toObject(message.position, options);
                if (message.clickCount != null && message.hasOwnProperty("clickCount"))
                    object.clickCount = message.clickCount;
                if (message.modifiers != null && message.hasOwnProperty("modifiers"))
                    object.modifiers = $root.zed.scene.Modifiers.toObject(message.modifiers, options);
                return object;
            };

            /**
             * Converts this MouseDownInput to JSON.
             * @function toJSON
             * @memberof zed.scene.MouseDownInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            MouseDownInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for MouseDownInput
             * @function getTypeUrl
             * @memberof zed.scene.MouseDownInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            MouseDownInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.MouseDownInput";
            };

            return MouseDownInput;
        })();

        scene.MouseUpInput = (function() {

            /**
             * Properties of a MouseUpInput.
             * @memberof zed.scene
             * @interface IMouseUpInput
             * @property {number|null} [button] MouseUpInput button
             * @property {zed.scene.IPoint|null} [position] MouseUpInput position
             * @property {zed.scene.IModifiers|null} [modifiers] MouseUpInput modifiers
             */

            /**
             * Constructs a new MouseUpInput.
             * @memberof zed.scene
             * @classdesc Represents a MouseUpInput.
             * @implements IMouseUpInput
             * @constructor
             * @param {zed.scene.IMouseUpInput=} [properties] Properties to set
             */
            function MouseUpInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * MouseUpInput button.
             * @member {number} button
             * @memberof zed.scene.MouseUpInput
             * @instance
             */
            MouseUpInput.prototype.button = 0;

            /**
             * MouseUpInput position.
             * @member {zed.scene.IPoint|null|undefined} position
             * @memberof zed.scene.MouseUpInput
             * @instance
             */
            MouseUpInput.prototype.position = null;

            /**
             * MouseUpInput modifiers.
             * @member {zed.scene.IModifiers|null|undefined} modifiers
             * @memberof zed.scene.MouseUpInput
             * @instance
             */
            MouseUpInput.prototype.modifiers = null;

            /**
             * Creates a new MouseUpInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {zed.scene.IMouseUpInput=} [properties] Properties to set
             * @returns {zed.scene.MouseUpInput} MouseUpInput instance
             */
            MouseUpInput.create = function create(properties) {
                return new MouseUpInput(properties);
            };

            /**
             * Encodes the specified MouseUpInput message. Does not implicitly {@link zed.scene.MouseUpInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {zed.scene.IMouseUpInput} message MouseUpInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MouseUpInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.button != null && Object.hasOwnProperty.call(message, "button"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.button);
                if (message.position != null && Object.hasOwnProperty.call(message, "position"))
                    $root.zed.scene.Point.encode(message.position, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.modifiers != null && Object.hasOwnProperty.call(message, "modifiers"))
                    $root.zed.scene.Modifiers.encode(message.modifiers, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified MouseUpInput message, length delimited. Does not implicitly {@link zed.scene.MouseUpInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {zed.scene.IMouseUpInput} message MouseUpInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MouseUpInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a MouseUpInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.MouseUpInput} MouseUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MouseUpInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.MouseUpInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.button = reader.uint32();
                            break;
                        }
                    case 2: {
                            message.position = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.modifiers = $root.zed.scene.Modifiers.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a MouseUpInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.MouseUpInput} MouseUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MouseUpInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a MouseUpInput message.
             * @function verify
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            MouseUpInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.button != null && message.hasOwnProperty("button"))
                    if (!$util.isInteger(message.button))
                        return "button: integer expected";
                if (message.position != null && message.hasOwnProperty("position")) {
                    let error = $root.zed.scene.Point.verify(message.position);
                    if (error)
                        return "position." + error;
                }
                if (message.modifiers != null && message.hasOwnProperty("modifiers")) {
                    let error = $root.zed.scene.Modifiers.verify(message.modifiers);
                    if (error)
                        return "modifiers." + error;
                }
                return null;
            };

            /**
             * Creates a MouseUpInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.MouseUpInput} MouseUpInput
             */
            MouseUpInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.MouseUpInput)
                    return object;
                let message = new $root.zed.scene.MouseUpInput();
                if (object.button != null)
                    message.button = object.button >>> 0;
                if (object.position != null) {
                    if (typeof object.position !== "object")
                        throw TypeError(".zed.scene.MouseUpInput.position: object expected");
                    message.position = $root.zed.scene.Point.fromObject(object.position);
                }
                if (object.modifiers != null) {
                    if (typeof object.modifiers !== "object")
                        throw TypeError(".zed.scene.MouseUpInput.modifiers: object expected");
                    message.modifiers = $root.zed.scene.Modifiers.fromObject(object.modifiers);
                }
                return message;
            };

            /**
             * Creates a plain object from a MouseUpInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {zed.scene.MouseUpInput} message MouseUpInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            MouseUpInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.button = 0;
                    object.position = null;
                    object.modifiers = null;
                }
                if (message.button != null && message.hasOwnProperty("button"))
                    object.button = message.button;
                if (message.position != null && message.hasOwnProperty("position"))
                    object.position = $root.zed.scene.Point.toObject(message.position, options);
                if (message.modifiers != null && message.hasOwnProperty("modifiers"))
                    object.modifiers = $root.zed.scene.Modifiers.toObject(message.modifiers, options);
                return object;
            };

            /**
             * Converts this MouseUpInput to JSON.
             * @function toJSON
             * @memberof zed.scene.MouseUpInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            MouseUpInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for MouseUpInput
             * @function getTypeUrl
             * @memberof zed.scene.MouseUpInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            MouseUpInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.MouseUpInput";
            };

            return MouseUpInput;
        })();

        scene.ScrollInput = (function() {

            /**
             * Properties of a ScrollInput.
             * @memberof zed.scene
             * @interface IScrollInput
             * @property {zed.scene.IPoint|null} [position] ScrollInput position
             * @property {zed.scene.IPoint|null} [delta] ScrollInput delta
             * @property {zed.scene.IModifiers|null} [modifiers] ScrollInput modifiers
             */

            /**
             * Constructs a new ScrollInput.
             * @memberof zed.scene
             * @classdesc Represents a ScrollInput.
             * @implements IScrollInput
             * @constructor
             * @param {zed.scene.IScrollInput=} [properties] Properties to set
             */
            function ScrollInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ScrollInput position.
             * @member {zed.scene.IPoint|null|undefined} position
             * @memberof zed.scene.ScrollInput
             * @instance
             */
            ScrollInput.prototype.position = null;

            /**
             * ScrollInput delta.
             * @member {zed.scene.IPoint|null|undefined} delta
             * @memberof zed.scene.ScrollInput
             * @instance
             */
            ScrollInput.prototype.delta = null;

            /**
             * ScrollInput modifiers.
             * @member {zed.scene.IModifiers|null|undefined} modifiers
             * @memberof zed.scene.ScrollInput
             * @instance
             */
            ScrollInput.prototype.modifiers = null;

            /**
             * Creates a new ScrollInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {zed.scene.IScrollInput=} [properties] Properties to set
             * @returns {zed.scene.ScrollInput} ScrollInput instance
             */
            ScrollInput.create = function create(properties) {
                return new ScrollInput(properties);
            };

            /**
             * Encodes the specified ScrollInput message. Does not implicitly {@link zed.scene.ScrollInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {zed.scene.IScrollInput} message ScrollInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ScrollInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.position != null && Object.hasOwnProperty.call(message, "position"))
                    $root.zed.scene.Point.encode(message.position, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.delta != null && Object.hasOwnProperty.call(message, "delta"))
                    $root.zed.scene.Point.encode(message.delta, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.modifiers != null && Object.hasOwnProperty.call(message, "modifiers"))
                    $root.zed.scene.Modifiers.encode(message.modifiers, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified ScrollInput message, length delimited. Does not implicitly {@link zed.scene.ScrollInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {zed.scene.IScrollInput} message ScrollInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ScrollInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a ScrollInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.ScrollInput} ScrollInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ScrollInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.ScrollInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.position = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.delta = $root.zed.scene.Point.decode(reader, reader.uint32());
                            break;
                        }
                    case 3: {
                            message.modifiers = $root.zed.scene.Modifiers.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a ScrollInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.ScrollInput} ScrollInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ScrollInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a ScrollInput message.
             * @function verify
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ScrollInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.position != null && message.hasOwnProperty("position")) {
                    let error = $root.zed.scene.Point.verify(message.position);
                    if (error)
                        return "position." + error;
                }
                if (message.delta != null && message.hasOwnProperty("delta")) {
                    let error = $root.zed.scene.Point.verify(message.delta);
                    if (error)
                        return "delta." + error;
                }
                if (message.modifiers != null && message.hasOwnProperty("modifiers")) {
                    let error = $root.zed.scene.Modifiers.verify(message.modifiers);
                    if (error)
                        return "modifiers." + error;
                }
                return null;
            };

            /**
             * Creates a ScrollInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.ScrollInput} ScrollInput
             */
            ScrollInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.ScrollInput)
                    return object;
                let message = new $root.zed.scene.ScrollInput();
                if (object.position != null) {
                    if (typeof object.position !== "object")
                        throw TypeError(".zed.scene.ScrollInput.position: object expected");
                    message.position = $root.zed.scene.Point.fromObject(object.position);
                }
                if (object.delta != null) {
                    if (typeof object.delta !== "object")
                        throw TypeError(".zed.scene.ScrollInput.delta: object expected");
                    message.delta = $root.zed.scene.Point.fromObject(object.delta);
                }
                if (object.modifiers != null) {
                    if (typeof object.modifiers !== "object")
                        throw TypeError(".zed.scene.ScrollInput.modifiers: object expected");
                    message.modifiers = $root.zed.scene.Modifiers.fromObject(object.modifiers);
                }
                return message;
            };

            /**
             * Creates a plain object from a ScrollInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {zed.scene.ScrollInput} message ScrollInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ScrollInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.position = null;
                    object.delta = null;
                    object.modifiers = null;
                }
                if (message.position != null && message.hasOwnProperty("position"))
                    object.position = $root.zed.scene.Point.toObject(message.position, options);
                if (message.delta != null && message.hasOwnProperty("delta"))
                    object.delta = $root.zed.scene.Point.toObject(message.delta, options);
                if (message.modifiers != null && message.hasOwnProperty("modifiers"))
                    object.modifiers = $root.zed.scene.Modifiers.toObject(message.modifiers, options);
                return object;
            };

            /**
             * Converts this ScrollInput to JSON.
             * @function toJSON
             * @memberof zed.scene.ScrollInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ScrollInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for ScrollInput
             * @function getTypeUrl
             * @memberof zed.scene.ScrollInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            ScrollInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.ScrollInput";
            };

            return ScrollInput;
        })();

        scene.KeyDownInput = (function() {

            /**
             * Properties of a KeyDownInput.
             * @memberof zed.scene
             * @interface IKeyDownInput
             * @property {string|null} [key] KeyDownInput key
             * @property {zed.scene.IModifiers|null} [modifiers] KeyDownInput modifiers
             */

            /**
             * Constructs a new KeyDownInput.
             * @memberof zed.scene
             * @classdesc Represents a KeyDownInput.
             * @implements IKeyDownInput
             * @constructor
             * @param {zed.scene.IKeyDownInput=} [properties] Properties to set
             */
            function KeyDownInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * KeyDownInput key.
             * @member {string} key
             * @memberof zed.scene.KeyDownInput
             * @instance
             */
            KeyDownInput.prototype.key = "";

            /**
             * KeyDownInput modifiers.
             * @member {zed.scene.IModifiers|null|undefined} modifiers
             * @memberof zed.scene.KeyDownInput
             * @instance
             */
            KeyDownInput.prototype.modifiers = null;

            /**
             * Creates a new KeyDownInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {zed.scene.IKeyDownInput=} [properties] Properties to set
             * @returns {zed.scene.KeyDownInput} KeyDownInput instance
             */
            KeyDownInput.create = function create(properties) {
                return new KeyDownInput(properties);
            };

            /**
             * Encodes the specified KeyDownInput message. Does not implicitly {@link zed.scene.KeyDownInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {zed.scene.IKeyDownInput} message KeyDownInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            KeyDownInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.key);
                if (message.modifiers != null && Object.hasOwnProperty.call(message, "modifiers"))
                    $root.zed.scene.Modifiers.encode(message.modifiers, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified KeyDownInput message, length delimited. Does not implicitly {@link zed.scene.KeyDownInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {zed.scene.IKeyDownInput} message KeyDownInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            KeyDownInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a KeyDownInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.KeyDownInput} KeyDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            KeyDownInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.KeyDownInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.key = reader.string();
                            break;
                        }
                    case 2: {
                            message.modifiers = $root.zed.scene.Modifiers.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a KeyDownInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.KeyDownInput} KeyDownInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            KeyDownInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a KeyDownInput message.
             * @function verify
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            KeyDownInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.key != null && message.hasOwnProperty("key"))
                    if (!$util.isString(message.key))
                        return "key: string expected";
                if (message.modifiers != null && message.hasOwnProperty("modifiers")) {
                    let error = $root.zed.scene.Modifiers.verify(message.modifiers);
                    if (error)
                        return "modifiers." + error;
                }
                return null;
            };

            /**
             * Creates a KeyDownInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.KeyDownInput} KeyDownInput
             */
            KeyDownInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.KeyDownInput)
                    return object;
                let message = new $root.zed.scene.KeyDownInput();
                if (object.key != null)
                    message.key = String(object.key);
                if (object.modifiers != null) {
                    if (typeof object.modifiers !== "object")
                        throw TypeError(".zed.scene.KeyDownInput.modifiers: object expected");
                    message.modifiers = $root.zed.scene.Modifiers.fromObject(object.modifiers);
                }
                return message;
            };

            /**
             * Creates a plain object from a KeyDownInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {zed.scene.KeyDownInput} message KeyDownInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            KeyDownInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.key = "";
                    object.modifiers = null;
                }
                if (message.key != null && message.hasOwnProperty("key"))
                    object.key = message.key;
                if (message.modifiers != null && message.hasOwnProperty("modifiers"))
                    object.modifiers = $root.zed.scene.Modifiers.toObject(message.modifiers, options);
                return object;
            };

            /**
             * Converts this KeyDownInput to JSON.
             * @function toJSON
             * @memberof zed.scene.KeyDownInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            KeyDownInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for KeyDownInput
             * @function getTypeUrl
             * @memberof zed.scene.KeyDownInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            KeyDownInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.KeyDownInput";
            };

            return KeyDownInput;
        })();

        scene.KeyUpInput = (function() {

            /**
             * Properties of a KeyUpInput.
             * @memberof zed.scene
             * @interface IKeyUpInput
             * @property {string|null} [key] KeyUpInput key
             * @property {zed.scene.IModifiers|null} [modifiers] KeyUpInput modifiers
             */

            /**
             * Constructs a new KeyUpInput.
             * @memberof zed.scene
             * @classdesc Represents a KeyUpInput.
             * @implements IKeyUpInput
             * @constructor
             * @param {zed.scene.IKeyUpInput=} [properties] Properties to set
             */
            function KeyUpInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * KeyUpInput key.
             * @member {string} key
             * @memberof zed.scene.KeyUpInput
             * @instance
             */
            KeyUpInput.prototype.key = "";

            /**
             * KeyUpInput modifiers.
             * @member {zed.scene.IModifiers|null|undefined} modifiers
             * @memberof zed.scene.KeyUpInput
             * @instance
             */
            KeyUpInput.prototype.modifiers = null;

            /**
             * Creates a new KeyUpInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {zed.scene.IKeyUpInput=} [properties] Properties to set
             * @returns {zed.scene.KeyUpInput} KeyUpInput instance
             */
            KeyUpInput.create = function create(properties) {
                return new KeyUpInput(properties);
            };

            /**
             * Encodes the specified KeyUpInput message. Does not implicitly {@link zed.scene.KeyUpInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {zed.scene.IKeyUpInput} message KeyUpInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            KeyUpInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.key);
                if (message.modifiers != null && Object.hasOwnProperty.call(message, "modifiers"))
                    $root.zed.scene.Modifiers.encode(message.modifiers, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };

            /**
             * Encodes the specified KeyUpInput message, length delimited. Does not implicitly {@link zed.scene.KeyUpInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {zed.scene.IKeyUpInput} message KeyUpInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            KeyUpInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a KeyUpInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.KeyUpInput} KeyUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            KeyUpInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.KeyUpInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.key = reader.string();
                            break;
                        }
                    case 2: {
                            message.modifiers = $root.zed.scene.Modifiers.decode(reader, reader.uint32());
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a KeyUpInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.KeyUpInput} KeyUpInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            KeyUpInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a KeyUpInput message.
             * @function verify
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            KeyUpInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.key != null && message.hasOwnProperty("key"))
                    if (!$util.isString(message.key))
                        return "key: string expected";
                if (message.modifiers != null && message.hasOwnProperty("modifiers")) {
                    let error = $root.zed.scene.Modifiers.verify(message.modifiers);
                    if (error)
                        return "modifiers." + error;
                }
                return null;
            };

            /**
             * Creates a KeyUpInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.KeyUpInput} KeyUpInput
             */
            KeyUpInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.KeyUpInput)
                    return object;
                let message = new $root.zed.scene.KeyUpInput();
                if (object.key != null)
                    message.key = String(object.key);
                if (object.modifiers != null) {
                    if (typeof object.modifiers !== "object")
                        throw TypeError(".zed.scene.KeyUpInput.modifiers: object expected");
                    message.modifiers = $root.zed.scene.Modifiers.fromObject(object.modifiers);
                }
                return message;
            };

            /**
             * Creates a plain object from a KeyUpInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {zed.scene.KeyUpInput} message KeyUpInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            KeyUpInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.key = "";
                    object.modifiers = null;
                }
                if (message.key != null && message.hasOwnProperty("key"))
                    object.key = message.key;
                if (message.modifiers != null && message.hasOwnProperty("modifiers"))
                    object.modifiers = $root.zed.scene.Modifiers.toObject(message.modifiers, options);
                return object;
            };

            /**
             * Converts this KeyUpInput to JSON.
             * @function toJSON
             * @memberof zed.scene.KeyUpInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            KeyUpInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for KeyUpInput
             * @function getTypeUrl
             * @memberof zed.scene.KeyUpInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            KeyUpInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.KeyUpInput";
            };

            return KeyUpInput;
        })();

        scene.ResizeInput = (function() {

            /**
             * Properties of a ResizeInput.
             * @memberof zed.scene
             * @interface IResizeInput
             * @property {zed.scene.ISize|null} [size] ResizeInput size
             * @property {number|null} [scaleFactor] ResizeInput scaleFactor
             */

            /**
             * Constructs a new ResizeInput.
             * @memberof zed.scene
             * @classdesc Represents a ResizeInput.
             * @implements IResizeInput
             * @constructor
             * @param {zed.scene.IResizeInput=} [properties] Properties to set
             */
            function ResizeInput(properties) {
                if (properties)
                    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ResizeInput size.
             * @member {zed.scene.ISize|null|undefined} size
             * @memberof zed.scene.ResizeInput
             * @instance
             */
            ResizeInput.prototype.size = null;

            /**
             * ResizeInput scaleFactor.
             * @member {number} scaleFactor
             * @memberof zed.scene.ResizeInput
             * @instance
             */
            ResizeInput.prototype.scaleFactor = 0;

            /**
             * Creates a new ResizeInput instance using the specified properties.
             * @function create
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {zed.scene.IResizeInput=} [properties] Properties to set
             * @returns {zed.scene.ResizeInput} ResizeInput instance
             */
            ResizeInput.create = function create(properties) {
                return new ResizeInput(properties);
            };

            /**
             * Encodes the specified ResizeInput message. Does not implicitly {@link zed.scene.ResizeInput.verify|verify} messages.
             * @function encode
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {zed.scene.IResizeInput} message ResizeInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ResizeInput.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.size != null && Object.hasOwnProperty.call(message, "size"))
                    $root.zed.scene.Size.encode(message.size, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.scaleFactor != null && Object.hasOwnProperty.call(message, "scaleFactor"))
                    writer.uint32(/* id 2, wireType 5 =*/21).float(message.scaleFactor);
                return writer;
            };

            /**
             * Encodes the specified ResizeInput message, length delimited. Does not implicitly {@link zed.scene.ResizeInput.verify|verify} messages.
             * @function encodeDelimited
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {zed.scene.IResizeInput} message ResizeInput message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ResizeInput.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a ResizeInput message from the specified reader or buffer.
             * @function decode
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {zed.scene.ResizeInput} ResizeInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ResizeInput.decode = function decode(reader, length, error) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end = length === undefined ? reader.len : reader.pos + length, message = new $root.zed.scene.ResizeInput();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    if (tag === error)
                        break;
                    switch (tag >>> 3) {
                    case 1: {
                            message.size = $root.zed.scene.Size.decode(reader, reader.uint32());
                            break;
                        }
                    case 2: {
                            message.scaleFactor = reader.float();
                            break;
                        }
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a ResizeInput message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {zed.scene.ResizeInput} ResizeInput
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ResizeInput.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a ResizeInput message.
             * @function verify
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ResizeInput.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.size != null && message.hasOwnProperty("size")) {
                    let error = $root.zed.scene.Size.verify(message.size);
                    if (error)
                        return "size." + error;
                }
                if (message.scaleFactor != null && message.hasOwnProperty("scaleFactor"))
                    if (typeof message.scaleFactor !== "number")
                        return "scaleFactor: number expected";
                return null;
            };

            /**
             * Creates a ResizeInput message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {zed.scene.ResizeInput} ResizeInput
             */
            ResizeInput.fromObject = function fromObject(object) {
                if (object instanceof $root.zed.scene.ResizeInput)
                    return object;
                let message = new $root.zed.scene.ResizeInput();
                if (object.size != null) {
                    if (typeof object.size !== "object")
                        throw TypeError(".zed.scene.ResizeInput.size: object expected");
                    message.size = $root.zed.scene.Size.fromObject(object.size);
                }
                if (object.scaleFactor != null)
                    message.scaleFactor = Number(object.scaleFactor);
                return message;
            };

            /**
             * Creates a plain object from a ResizeInput message. Also converts values to other types if specified.
             * @function toObject
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {zed.scene.ResizeInput} message ResizeInput
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ResizeInput.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                let object = {};
                if (options.defaults) {
                    object.size = null;
                    object.scaleFactor = 0;
                }
                if (message.size != null && message.hasOwnProperty("size"))
                    object.size = $root.zed.scene.Size.toObject(message.size, options);
                if (message.scaleFactor != null && message.hasOwnProperty("scaleFactor"))
                    object.scaleFactor = options.json && !isFinite(message.scaleFactor) ? String(message.scaleFactor) : message.scaleFactor;
                return object;
            };

            /**
             * Converts this ResizeInput to JSON.
             * @function toJSON
             * @memberof zed.scene.ResizeInput
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ResizeInput.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            /**
             * Gets the default type url for ResizeInput
             * @function getTypeUrl
             * @memberof zed.scene.ResizeInput
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            ResizeInput.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/zed.scene.ResizeInput";
            };

            return ResizeInput;
        })();

        return scene;
    })();

    return zed;
})();

export { $root as default };
