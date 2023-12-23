#[doc = "Register `sd_host_ctrl_ver` reader"]
pub struct R(crate::R<SD_HOST_CTRL_VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_HOST_CTRL_VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_HOST_CTRL_VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_HOST_CTRL_VER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_host_ctrl_ver` writer"]
pub struct W(crate::W<SD_HOST_CTRL_VER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_HOST_CTRL_VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_HOST_CTRL_VER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_HOST_CTRL_VER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sd_ver` reader - "]
pub type SD_VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vendor_ver` reader - "]
pub type VENDOR_VER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd_ver(&self) -> SD_VER_R {
        SD_VER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn vendor_ver(&self) -> VENDOR_VER_R {
        VENDOR_VER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control Version Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_host_ctrl_ver](index.html) module"]
pub struct SD_HOST_CTRL_VER_SPEC;
impl crate::RegisterSpec for SD_HOST_CTRL_VER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_host_ctrl_ver::R](R) reader structure"]
impl crate::Readable for SD_HOST_CTRL_VER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_host_ctrl_ver::W](W) writer structure"]
impl crate::Writable for SD_HOST_CTRL_VER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_host_ctrl_ver to value 0x02"]
impl crate::Resettable for SD_HOST_CTRL_VER_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
