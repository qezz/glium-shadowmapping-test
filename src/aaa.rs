
        let biasMatrix: cgmath::Matrix4<f32> = [[ 0.5_f32, 0.0, 0.0, 0.0, ],
                                                [ 0.0, 0.5, 0.0, 0.0, ],
                                                [ 0.0, 0.0, 0.5, 0.0, ],
                                                [ 0.5, 0.5, 0.5, 1.0, ]].into();

        let depthMVP = depthProjectionMatrix * depthViewMatrix * depthModelMatrix;

        let depthBiasMVP = biasMatrix * depthMVP;
