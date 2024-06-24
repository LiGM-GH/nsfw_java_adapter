class NsfwPredictor {
    private static native boolean predict(String filename);

    static {
        System.loadLibrary("nsfw_lib");
    }

    public static void main(String[] args) {
        boolean output = NsfwPredictor.predict("test-images/nsfw_image_1.jpeg");
        System.out.println(output);
    }
}
